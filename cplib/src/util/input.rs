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
    
    pub fn read_eof() {
        use std::io::*;
        let s = Box::leak(read_to_string(stdin()).unwrap().into_boxed_str());
        let iter = s.split_whitespace();
        ITER.set_global(iter);
    }
    
    pub fn read() {
        if crate::LOCAL { Self::read_line(); } else { Self::read_eof(); }
    }
    
    pub fn char() -> char { next() }
    pub fn usize() -> usize { next() }
    pub fn usize1() -> usize { Scan::usize()-1 }
    pub fn u64() -> u64 { next() }
    pub fn i64() -> i64 { next() }
    pub fn string() -> String { next() }
    pub fn chars() -> Vec<char> { next::<String>().chars().collect() }
    pub fn vec<T: std::str::FromStr>(n: usize) -> Vec<T> { (0..n).map(|_| next()).collect() }
    pub fn vusize(n: usize) -> Vec<usize> { Scan::vec(n) }
    pub fn vu64(n: usize) -> Vec<u64> { Scan::vec(n) }
    pub fn vi64(n: usize) -> Vec<i64> { Scan::vec(n) }
}

#[macro_export]
macro_rules! VEC {
    ($t:ty; $n:expr) => {
        Scan::vec::<$t>($n)
    };
}
