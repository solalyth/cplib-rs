//! Rolling Hash
//! 
//! 法を `2^61 - 1` とし、基数として `37, 43` を採用した。

use std::ops::{Add, Shl, Shr, Sub};



pub const P: u128 = (1<<61)-1;
pub const fn reduce(mut v: u64) -> u64 { if P as u64 <= v { v -= P as u64; } v }
pub const fn rem(v: u128) -> u64 { reduce(((v >> 61)+(v & P)) as u64) }

macro_rules! mul {
    ($lhs:expr, $rhs:expr, $T:ty) => {
        [rem($lhs[0] as u128 * $rhs[0] as u128) as $T, rem($lhs[1] as u128 * $rhs[1] as u128) as $T]
    };
}

const BASE: [u128; 2] = [37, 43];
const INV: [u128; 2] = [2181202846553494278, 965236608508057933];

// fast pow: 2^B 進数で C 個に分割する。2^BC まで計算可能。
const B: usize = 8;
const C: usize = 8;

macro_rules! impl_fast_pow {
    ($base:expr, $table:ident, $fn_name:ident) => {
        const $table: [[u128; 2]; C<<B] = {
            let (mut res, mut x, mut i) = ([[1; 2]; C<<B], $base, 0);
            
            while i < C {
                let (mut y, mut j) = ([1; 2], 0);
                while j>>B == 0 {
                    res[(i<<B)+j] = y;
                    y = mul!(x, y, u128);
                    j += 1;
                }
                x = y;
                i += 1;
            }
            res
        };
        
        pub const fn $fn_name(mut x: [u64; 2], mut exp: usize) -> [u64; 2] {
            assert!(64 <= B*C || exp < 1<<B*C);
            
            let mut c = 0;
            while exp != 0 {
                x = mul!(x, $table[(exp&((1<<B)-1)) + (c<<B)], u64);
                exp >>= B;
                c += 1;
            }
            x
        }
    };
}



impl_fast_pow!(BASE, BASE_TABLE, base_pow);
impl_fast_pow!(INV, INV_TABLE, inv_pow);



/// Rolling Hash
/// 
/// # Features
/// 
/// - `Add, Sub, Shl<usize>, Shr<usize>`
/// - `Deref<Target = [u64; 2]>`
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct Hash(pub(crate) [u64; 2]);

impl Hash {
    /// # Panics
    /// 
    /// - if not `v < P*(P+1)`
    pub const fn new(v: u64) -> Hash { assert!(v < (P*(P+1)) as u64); Hash([rem(v as u128); 2]) }
    
    /// `[A] -> [A, v]`
    pub fn push(self, v: u64) -> Hash { Hash(mul!(self.0, BASE, u64)) + Hash::new(v) }
    
    /// `[A, v] -> [A]`
    pub fn pop(self, v: u64) -> Hash { self-Hash::new(v) >> 1 }
    
    pub fn concat(self, r: Self, rlen: usize) -> Self { (self << rlen) + r }
    
    pub fn inner(&self) -> &[u64; 2] { &self.0 }
    
    pub fn prefix_fold(iter: impl IntoIterator<Item = u64>) -> Vec<Hash> {
        let (mut res, mut cur) = (vec![Hash::new(0)], Hash::new(0));
        for x in iter.into_iter() { cur = cur.push(x); res.push(cur); }
        res
    }
}


impl Add for Hash {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output { for i in 0..2 { self.0[i] = reduce(self.0[i] + rhs.0[i]); } self }
}

impl Sub for Hash {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output { for i in 0..2 { self.0[i] = reduce(P as u64 + self.0[i] - rhs.0[i]); } self }
}

impl Shl<usize> for Hash {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self::Output { Self(base_pow(self.0, rhs)) }
}

impl Shr<usize> for Hash {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self::Output { Self(inv_pow(self.0, rhs)) }
}



/// `&[Hash]` として `[..=lcp]` まで一致していることを表す。すなわち、平文として `[..lcp]` まで一致している。
pub fn lcp(s: &[Hash], t: &[Hash]) -> usize {
    let (mut ok, mut ng) = (0, s.len().min(t.len())+1);
    while ng-ok > 1 {
        let x = (ng+ok)/2;
        if s[x] == t[x] { ok = x; } else { ng = x; }
    }
    ok
}


pub fn slice_cmp(s: &[Hash], t: &[Hash]) -> std::cmp::Ordering {
    let lcp = lcp(s, t);
    if s.len().min(t.len()) == lcp+1 {
        s.len().cmp(&t.len())
    } else {
        (t[lcp+1]-s[lcp+1]).0[0].cmp(&(P as u64/2))
    }
}

fn concat_get(s: &[Hash], t: &[Hash], len: usize) -> Hash {
    if s.len() < len { (s[s.len()] << len-s.len()) + t[len-s.len()] } else { s[len] }
}

pub fn concat_cmp(s: &[Hash], t: &[Hash]) -> std::cmp::Ordering {
    // st.cmp(ts)
    let (mut ng, mut ok) = (0, s.len()+t.len()-1);
    while ok-ng > 1 {
        let x = (ng+ok)/2;
        let st = concat_get(s, t, x);
        let ts = concat_get(t, s, x);
        if st == ts { ok = x; } else { ng = x; }
    }
    if ok == s.len()+t.len()-1 { std::cmp::Ordering::Equal } else { (concat_get(t, s, ok+1) - concat_get(s, t, ok+1)).0[0].cmp(&(P as u64/2)) }
}
