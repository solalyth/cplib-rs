//! Rolling Hash
//! 
//! 法を `2^61 - 1` とし、基数として `37, 43` を採用した。

use std::ops::{Add, Deref, Shl, Shr, Sub};



pub const MOD: u128 = 2u128.pow(61)-1;
pub const fn reduce(mut v: u64) -> u64 { if MOD as u64 <= v { v -= MOD as u64; } v }
pub const fn rem(v: u128) -> u64 { reduce(((v >> 61)+(v & MOD)) as u64) }

const BASE: [u128; 2] = [37, 43];
const INV: [u128; 2] = [2181202846553494278, 965236608508057933];
const POW_LEN: usize = 30;

const BASE_EXP: [[u128; 2]; POW_LEN] = {
    let mut res = [BASE; POW_LEN];
    let mut i = 1;
    while i < POW_LEN { res[i] = [res[i-1][0].pow(2)%MOD, res[i-1][1].pow(2)%MOD]; i += 1; }
    res
};
const BASEINV_EXP: [[u128; 2]; POW_LEN] = {
    let mut res = [INV; POW_LEN];
    let mut i = 1;
    while i < POW_LEN { res[i] = [res[i-1][0].pow(2)%MOD, res[i-1][1].pow(2)%MOD]; i += 1; }
    res
};



pub fn base_pow(e: usize) -> [u128; 2] {
    assert!(e < 1<<POW_LEN);
    let mut res = [1; 2];
    for i in 0..POW_LEN {
        if e>>i == 0 { break; }
        if e>>i&1 == 1 { for j in 0..2 { res[j] = rem(res[j]*BASE_EXP[i][j]) as u128; } }
    }
    res
}

pub fn base_invpow(e: usize) -> [u128; 2] {
    assert!(e < 1<<POW_LEN);
    let mut res = [1; 2];
    for i in 0..POW_LEN {
        if e>>i == 0 { break; }
        if e>>i&1 == 1 { for j in 0..2 { res[j] = rem(res[j]*BASEINV_EXP[i][j]) as u128; } }
    }
    res
}



/// Rolling Hash
/// 
/// # Features
/// 
/// - `Add, Sub, Shl<usize>, Shr<usize>`
/// - `Deref<Target = [u64; 2]>`
/// - `FromIterator<u64>`
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct Hash(pub(crate) [u64; 2]);

impl Hash {
    /// # Panics
    /// 
    /// - if not `v < MOD = 2^61 - 1`
    pub const fn new(v: u64) -> Hash { assert!(v < MOD as u64); Hash([rem(v as u128); 2]) }
    
    /// `[A] -> [A, v]`
    /// 
    /// # Panics
    /// 
    /// - if not `v < MOD = 2^61 - 1`
    pub fn push(self, v: u64) -> Hash { (self << 1) + Hash::new(v) }
    
    /// `[A, v] -> [A]`
    /// 
    /// # Panics
    /// 
    /// - if not `v < MOD = 2^61 - 1`
    pub fn push_inv(self, v: u64) -> Hash { self-Hash::new(v) >> 1 }
    
    pub fn concat(self, r: Self, rlen: usize) -> Self { (self << rlen) + r }
    
    pub fn inner(&self) -> &[u64; 2] { &self.0 }
}


impl Add for Hash {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..2 { self.0[i] = reduce(self.0[i] + rhs.0[i]); }
        self
    }
}

impl Sub for Hash {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..2 { self.0[i] = reduce(MOD as u64 + self.0[i] - rhs.0[i]); }
        self
    }
}

impl Shl<usize> for Hash {
    type Output = Self;
    fn shl(mut self, rhs: usize) -> Self::Output {
        let bp = base_pow(rhs);
        for i in 0..2 { self.0[i] = rem(self.0[i] as u128 * bp[i]); }
        self
    }
}

impl Shr<usize> for Hash {
    type Output = Self;
    fn shr(mut self, rhs: usize) -> Self::Output {
        let bp = base_invpow(rhs);
        for i in 0..2 { self.0[i] = rem(self.0[i] as u128 * bp[i]); }
        self
    }
}

impl Deref for Hash {
    type Target = [u64; 2];
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl FromIterator<u64> for Hash {
    fn from_iter<T: IntoIterator<Item = u64>>(iter: T) -> Self {
        iter.into_iter().fold(Hash::new(0), |acc, v| acc.push(v))
    }
}
