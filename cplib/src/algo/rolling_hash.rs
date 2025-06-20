//! Rolling Hash
//! 
//! この実装では法を `2^61 - 1` とし、基数(原始根)として `1000007, 133` を採用した。

use std::collections::VecDeque;
use std::ops::{Add, Neg, RangeBounds, Shl, Shr, Sub};
use std::cmp::Ordering;

use crate::cplib::util::func::to_bounds;

pub const MOD: u128 = 2u128.pow(61)-1;
pub const fn reduce(mut v: u64) -> u64 { if MOD as u64 <= v { v -= MOD as u64; } v }
pub const fn rem(v: u128) -> u64 { reduce(((v >> 61)+(v & MOD)) as u64) }

const BASE: [u128; 2] = [37, 43];
const INV: [u128; 2] = [2181202846553494278, 965236608508057933];
const POW_LEN: usize = 30; // 2^30 = 1e9 の長さなら可能

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
    // if e == 1 { return BASE; }
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



#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Hash([u64; 2]);

impl Hash {
    pub fn new(v: u64) -> Hash { Hash([rem(v as u128); 2]) }
    
    pub fn push(self, v: u64) -> Hash { (self << 1) + v }
    pub fn push_inv(self, v: u64) -> Hash { self-v >> 1 }
    
    pub fn fold(iter: impl Iterator<Item = u64>) -> Hash {
        iter.fold(Hash::new(0), |acc, v| acc.push(v))
    }
    
    pub fn prod(lhs: Self, rhs: Self, rl: usize) -> Self { (lhs << rl) + rhs }
}


impl Neg for Hash {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        for i in 0..2 { self.0[i] = reduce(MOD as u64 - self.0[i]); }
        self
    }
}

impl Add for Hash {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..2 { self.0[i] = reduce(self.0[i] + rhs.0[i]); }
        self
    }
}

impl Add<u64> for Hash {
    type Output = Self;
    fn add(self, rhs: u64) -> Self::Output {
        self + Hash::new(rhs)
    }
}

impl Sub for Hash {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..2 { self.0[i] = reduce(MOD as u64 + self.0[i] - rhs.0[i]); }
        self
    }
}

impl Sub<u64> for Hash {
    type Output = Self;
    fn sub(self, rhs: u64) -> Self::Output {
        self - Hash::new(rhs)
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





pub struct HashDeque {
    s: VecDeque<char>,
    prefix: VecDeque<Hash>
}

impl HashDeque {
    pub fn new(s: VecDeque<char>) -> Self {
        let mut prefix = VecDeque::<Hash>::from([Hash::new(0)]);
        for i in 0..s.len() { prefix.push_back(prefix[i].push(s[i] as u64)); }
        Self { s, prefix }
    }
    
    pub fn len(&self) -> usize {
        self.s.len()
    }
    
    pub fn push_back(&mut self, c: char) {
        self.prefix.push_back(self.prefix[self.prefix.len()-1].push(c as u64));
        self.s.push_back(c);
    }
    
    pub fn push_front(&mut self, c: char) {
        self.prefix.push_front(self.prefix[0].push_inv(c as u64));
        self.s.push_front(c);
    }
    
    pub fn slice<'a>(&'a self, range: impl RangeBounds<usize>) -> HashDequeSlice<'a> {
        let [l, r] = to_bounds(range, self.len());
        HashDequeSlice { ptr: self, l, r }
    }
    
    pub fn fold(&self, range: impl RangeBounds<usize>) -> Hash {
        let [l, r] = to_bounds(range, self.len());
        self.prefix[r] - (self.prefix[l] << r-l)
    }
}

impl PartialEq for HashDeque {
    fn eq(&self, other: &Self) -> bool {
        self.slice(..).eq(&other.slice(..))
    }
}
impl Ord for HashDeque {
    fn cmp(&self, other: &Self) -> Ordering {
        self.slice(..).cmp(&other.slice(..))
    }
}
impl Eq for HashDeque {}
impl PartialOrd for HashDeque { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.cmp(other).into() } }




pub struct HashDequeSlice<'a> {
    ptr: &'a HashDeque,
    l: usize,
    r: usize
}

impl<'a> HashDequeSlice<'a> {
    fn len(&self) -> usize {
        self.r - self.l
    }
    
    fn prefix(&self, len: usize) -> Hash {
        assert!(len <= self.len());
        self.ptr.fold(self.l..self.l+len)
    }
}

impl<'a> PartialEq for HashDequeSlice<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.prefix(self.len()) == other.prefix(self.len())
    }
}

impl<'a> Ord for HashDequeSlice<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let (sl, tl) = (self.len(), other.len());
        let ml = sl.min(tl);
        if self.prefix(ml) == other.prefix(ml) { return sl.cmp(&tl); }
        let (mut ng, mut ok) = (0, ml);
        while 1 < ok-ng {
            let mid = (ng+ok)/2;
            if self.prefix(mid) == other.prefix(mid) { ng = mid; } else { ok = mid; }
        }
        self.ptr.s[ok].cmp(&other.ptr.s[ok])
    }
}

impl<'a> Eq for HashDequeSlice<'a> {}
impl<'a> PartialOrd for HashDequeSlice<'a> { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.cmp(other).into() } }
