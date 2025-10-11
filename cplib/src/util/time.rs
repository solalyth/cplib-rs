use std::sync::OnceLock;
use std::time::Instant;

pub(crate) static TIME: OnceLock<Instant> = OnceLock::new();

pub fn init() {
    TIME.set(Instant::now()).unwrap();
}

pub fn wait(t: u128) {
    if let Some(ins) = TIME.get() {
        while ins.elapsed().as_millis() < t {}
    }
}

pub fn assert_wait(v: bool, t: u128) {
    if !v {
        if crate::SUBMISSION { wait(t); }
        panic!();
    }
}
