//! Exit requests are acted on after presentation, outside button/drawing code.
#[derive(Default)]
pub struct Shutdown {
    requested: bool,
    frame_presented: bool,
}
impl Shutdown {
    pub fn request(&mut self) {
        self.requested = true;
    }
    pub fn frame_presented(&mut self) {
        if self.requested {
            self.frame_presented = true;
        }
    }
    pub fn should_exit(&self) -> bool {
        self.requested && self.frame_presented
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exit_waits_until_the_current_frame_has_finished() {
        let mut s = Shutdown::default();
        s.request();
        assert!(!s.should_exit());
        s.frame_presented();
        assert!(s.should_exit());
    }
    #[test]
    fn normal_frames_do_not_exit_and_repeated_requests_are_safe() {
        let mut s = Shutdown::default();
        s.frame_presented();
        assert!(!s.should_exit());
        s.request();
        s.request();
        assert!(!s.should_exit());
        s.frame_presented();
        assert!(s.should_exit());
        s.request();
        assert!(s.should_exit());
    }
}
