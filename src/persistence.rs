//! Save I/O runs on one worker; the rendering thread only queues snapshots.
use crate::save::Save;
use std::{
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
    time::Duration,
};
enum Message {
    Save(Save),
    Flush(Sender<Result<(), String>>),
    Stop,
}
pub struct SaveWriter {
    tx: Sender<Message>,
    errors: Receiver<String>,
}
impl SaveWriter {
    pub fn new(path: PathBuf) -> Self {
        Self::spawn(move |save| save.write(&path).map_err(|e| e.to_string()))
    }
    fn spawn(write: impl Fn(&Save) -> Result<(), String> + Send + 'static) -> Self {
        let (tx, rx) = mpsc::channel();
        let (error_tx, errors) = mpsc::channel();
        std::thread::Builder::new()
            .name("chess-save".into())
            .spawn(move || {
                let mut result = Ok(());
                while let Ok(message) = rx.recv() {
                    match message {
                        Message::Save(mut save) => {
                            let mut barrier = None;
                            let mut stop = false;
                            while let Ok(next) = rx.try_recv() {
                                match next {
                                    Message::Save(new) => save = new,
                                    Message::Flush(reply) => {
                                        barrier = Some(reply);
                                        break;
                                    }
                                    Message::Stop => {
                                        stop = true;
                                        break;
                                    }
                                }
                            }
                            result = write(&save);
                            if let Err(e) = &result {
                                let _ = error_tx.send(e.clone());
                            }
                            if let Some(reply) = barrier {
                                let _ = reply.send(result.clone());
                            }
                            if stop {
                                break;
                            }
                        }
                        Message::Flush(reply) => {
                            let _ = reply.send(result.clone());
                        }
                        Message::Stop => break,
                    }
                }
            })
            .expect("Save worker");
        Self { tx, errors }
    }
    pub fn queue(&self, save: &Save) -> Result<(), String> {
        self.tx
            .send(Message::Save(save.clone()))
            .map_err(|e| e.to_string())
    }
    pub fn error(&self) -> Option<String> {
        self.errors.try_recv().ok()
    }
    /// Called once at normal shutdown; bound disk failures to three seconds.
    pub fn flush(&self) -> Result<(), String> {
        let (tx, rx) = mpsc::channel();
        self.tx
            .send(Message::Flush(tx))
            .map_err(|e| e.to_string())?;
        rx.recv_timeout(Duration::from_secs(3))
            .map_err(|e| e.to_string())?
    }
}
impl Drop for SaveWriter {
    fn drop(&mut self) {
        let _ = self.tx.send(Message::Stop);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn disk_writes_run_off_the_calling_thread_and_flush_keeps_the_last_snapshot() {
        let (tx, rx) = mpsc::channel();
        let caller = std::thread::current().id();
        let writer = SaveWriter::spawn(move |s| {
            tx.send((std::thread::current().id(), s.clone())).unwrap();
            Ok(())
        });
        let mut save = Save::default();
        for i in 0..20 {
            save.volume = i as f32 / 20.;
            writer.queue(&save).unwrap();
        }
        writer.flush().unwrap();
        let results: Vec<_> = rx.try_iter().collect();
        assert!(!results.is_empty());
        assert!(results.iter().all(|(id, _)| *id != caller));
        assert_eq!(results.last().unwrap().1, save);
    }
    #[test]
    fn asynchronous_errors_are_reported_and_flush_does_not_claim_success() {
        let writer = SaveWriter::spawn(|_| Err("disk unavailable".into()));
        writer.queue(&Save::default()).unwrap();
        assert_eq!(writer.flush(), Err("disk unavailable".into()));
        assert_eq!(writer.error(), Some("disk unavailable".into()));
    }
}
