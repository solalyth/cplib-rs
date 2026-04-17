use crate::cplib::util::global::Global;

static ITER: Global<std::str::SplitWhitespace<'static>> = Global::new();

fn next<T: std::str::FromStr>() -> T {
    loop {
        let iter = ITER.get_mut_global();
        if let Some(s) = iter.next() {
            return s.parse().ok().unwrap();
        } else {
            Scan::read_line();
        }
    }
}


pub struct Scan;

impl Scan {
    pub fn read_line() {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let s = Box::leak(buf.into_boxed_str());
        let iter = s.split_whitespace();
        ITER.set_global(iter);
    }
    
    pub fn usize() -> usize { next() }
    pub fn u64() -> u64 { next() }
    pub fn i64() -> i64 { next() }
    pub fn string() -> String { next() }
    pub fn chars() -> Vec<char> { next::<String>().chars().collect() }
    pub fn vec<T: std::str::FromStr>(n: usize) -> Vec<T> { (0..n).map(|_| next()).collect() }
}
