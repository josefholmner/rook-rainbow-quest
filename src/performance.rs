//! Opt-in native profiling. Reports use isolated saves; no screenshots/PNG work.
use std::{
    cell::{Cell, RefCell},
    path::Path,
    time::Instant,
};
thread_local! {
    static ENABLED:Cell<bool>=const{Cell::new(false)};
    static FRAME:Cell<u32>=const{Cell::new(0)};
    static SAMPLES:RefCell<Vec<(&'static str,u32,f64)>>=const{RefCell::new(Vec::new())};
}
pub fn enable() {
    ENABLED.with(|v| v.set(true));
}
pub fn enabled() -> bool {
    ENABLED.with(Cell::get)
}
pub fn frame(n: u32) {
    FRAME.with(|v| v.set(n));
}
pub fn begin() -> Option<Instant> {
    enabled().then(Instant::now)
}
pub fn end(name: &'static str, start: Option<Instant>) {
    if let Some(t) = start {
        let ms = t.elapsed().as_secs_f64() * 1000.;
        let n = FRAME.with(Cell::get);
        SAMPLES.with(|v| v.borrow_mut().push((name, n, ms)));
    }
}
pub fn report(path: &Path) {
    SAMPLES.with(|v|{
        let samples=v.borrow();let mut groups=std::collections::BTreeMap::<String,Vec<f64>>::new();
        for (name,n,ms) in samples.iter(){
            groups.entry((*name).into()).or_default().push(*ms);
            if *name=="frame" {groups.entry(if *n<480{"cold_outward"}else{"warm_return"}.into()).or_default().push(*ms);}
        }
        let summaries:std::collections::BTreeMap<_,_>=groups.into_iter().map(|(name,mut values)|{
            values.sort_by(f64::total_cmp);let count=values.len();
            let q=|p:f64|values[((count-1) as f64*p).round() as usize];
            (name,serde_json::json!({"count":count,"p50_ms":q(0.5),"p95_ms":q(0.95),"p99_ms":q(0.99),"max_ms":q(1.),"over_25ms":values.iter().filter(|v|**v>25.).count(),"over_50ms":values.iter().filter(|v|**v>50.).count()}))
        }).collect();
        if let Some(parent)=path.parent(){std::fs::create_dir_all(parent).unwrap();}
        std::fs::write(path,serde_json::to_vec_pretty(&serde_json::json!({"version":env!("CARGO_PKG_VERSION"),"summaries":summaries,"samples":*samples})).unwrap()).unwrap();
        println!("Performance report: {}",path.display());
    });
}
