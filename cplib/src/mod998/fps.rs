use std::{fmt::Debug, ops::{Deref, DerefMut, Mul}, slice::SliceIndex};

use crate::cplib::mod998::fp::Fp;


/// `ROOT[i]` には `1` の `2^(i+1)` 乗根が入っている。`15311432` は `2^23` 乗根の一つ。
const ROOT: [Fp; 23] = {
    let (mut pow, mut cur, mut i) = ([Fp::raw(0); 23], Fp::raw(15311432), 23);
    while i != 0 {
        i -= 1;
        pow[i] = cur;
        cur = cur.mul(cur);
    }
    pow
};

const ROOT_INV: [Fp; 23] = {
    let (mut pow, mut cur, mut i) = ([Fp::raw(0); 23], Fp::raw(15311432).inv(), 23);
    while i != 0 {
        i -= 1;
        pow[i] = cur;
        cur = cur.mul(cur);
    }
    pow
};



pub struct Fps(Vec<Fp>);

impl Fps {
    pub fn new() -> Self { Fps(vec![]) }
    pub fn to_vec(self) -> Vec<Fp> { self.0 }
    
    pub fn convolution(&self, rhs: &Self) -> Self {
        Fps(convolution(&self.0, &rhs.0))
    }
    
    /// `1/f` を計算する。
    /// 
    /// # Panic
    /// 
    /// - if `f[0] == 0`
    pub fn inv(&self, n: usize) -> Self {
        assert!(self.0[0] != Fp::raw(0));
        let mut res = inv(&self.0, ilog2_ceil(n));
        res.truncate(n);
        Fps(res)
    }
    
    pub fn truncate_zero(&mut self) {
        for i in (1..self.0.len()).rev() {
            if self.0[i] == Fp::raw(0) { self.0.pop(); } else { break; }
        }
    }
    
    pub fn deg(&self) -> usize {
        self.0.len() - 1
    }
}

impl Mul for &Fps {
    type Output = Fps;
    fn mul(self, rhs: Self) -> Self::Output { self.convolution(rhs) }
}

impl Debug for Fps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Deref for Fps {
    type Target = Vec<Fp>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Fps {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl <I: SliceIndex<[Fp]>> std::ops::Index<I> for Fps {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output { &self.0[index] }
}



/// サイズ `2^log` の DFT を計算する。
fn fft(f: &[Fp], log: usize) -> Vec<Fp> {
    if log == 0 { return vec![f[0]]; }
    
    let mut a = vec![Fp::raw(0); 1<<log];
    for i in 0..f.len() { let idx = bitrev(i, log); a[idx] = a[idx] + f[i]; }
    
    for d in 0..log {
        for w in a.chunks_exact_mut(1<<d+1) {
            let mut p = Fp::raw(1);
            for i in 0..1<<d {
                let t = w[i+(1<<d)] * p;
                w[i+(1<<d)] = w[i] - t;
                w[i] = w[i] + t;
                p = p.mul(ROOT[d]);
            }
        }
    }
    
    a
}

/// サイズ `2^log` の iDFT を計算する。
fn ifft(fft: &[Fp], log: usize) -> Vec<Fp> {
    if log == 0 { return vec![fft[0]]; }
    
    let mut a = vec![Fp::raw(0); 1<<log];
    for i in 0..fft.len() { let idx = bitrev(i, log); a[idx] = a[idx] + fft[i]; }
    
    for d in 0..log {
        for w in a.chunks_exact_mut(1<<d+1) {
            let mut p = Fp::raw(1);
            for i in 0..1<<d {
                let t = w[i+(1<<d)] * p;
                w[i+(1<<d)] = w[i] - t;
                w[i] = w[i] + t;
                p = p.mul(ROOT_INV[d]);
            }
        }
    }
    
    let inv_n = Fp::raw(1<<log).inv();
    for x in &mut a { *x = *x * inv_n; }
    
    a
}

/// [`Fps::convolution`]
fn convolution(f: &[Fp], g: &[Fp]) -> Vec<Fp> {
    let n = f.len() + g.len() - 1;
    if f.len().min(g.len()) <= 64 {
        let mut res = vec![Fp::raw(0); n];
        for i in 0..f.len() {
            for j in 0..g.len() {
                res[i+j] = res[i+j] + f[i]*g[j];
            }
        }
        res
    } else {
        let log = ilog2_ceil(n);
        let (mut f, g) = (fft(&f, log), fft(&g, log));
        for i in 0..1<<log { f[i] = f[i] * g[i]; }
        let mut f = ifft(&f, log);
        f.truncate(n);
        f
    }
}

/// [`Fps::inv`]
/// 
/// # Constraints
/// 
/// - `f[0] != 0`
fn inv(f: &[Fp], log: usize) -> Vec<Fp> {
    let mut cur = vec![Fp::raw(0); 1<<log];
    cur[0] = f[0].inv();
    for d in 0..log {
        let mut f = fft(&f[..f.len().min(1<<d+1)], d+1);
        let g = fft(&cur[..1<<d], d+1);
        for i in 0..1<<d+1 { f[i] = f[i] * g[i]; }
        f = ifft(&f, d+1);
        for i in 0..1<<d { f[i] = Fp::raw(0); }
        f = fft(&f, d+1);
        for i in 0..1<<d+1 { f[i] = f[i] * g[i]; }
        f = ifft(&f, d+1);
        for i in 1<<d..1<<d+1 { cur[i] = -f[i]; }
    }
    
    cur
}


const fn bitrev(x: usize, log: usize) -> usize { x.reverse_bits() as usize >> (64-log) }
const fn ilog2_ceil(x: usize) -> usize { if x == 1 { 0 } else { (x-1).ilog2() as usize + 1 } }
