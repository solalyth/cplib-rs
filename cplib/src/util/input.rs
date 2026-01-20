use std::io::Read;



pub struct Scan {
    iter: std::str::SplitAsciiWhitespace<'static>,
}

impl Scan {
    pub fn new() -> Self {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();
        let s = Box::leak(buf.into_boxed_str());
        let iter = s.split_ascii_whitespace();
        Scan { iter }
    }
    
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        self.iter.next().unwrap().parse().ok().unwrap()
    }
    
    pub fn usize(&mut self) -> usize { self.next() }
    pub fn i64(&mut self) -> i64 { self.next() }
    
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next()).collect()
    }
    
    pub fn chars(&mut self) -> Vec<char> {
        self.iter.next().unwrap().chars().collect()
    }
}
