//! struct [`HashDeque`]
//! 
//! # Todo
//! 
//! いつか `HashDequeSlice` と比較を書く。
//! 
//! # Requires
//! 
//! - `algo/rolling_hash`
//! - `util/func`

use crate::cplib::{algo::rolling_hash::{Hash, MOD}, util::func::to_bounds};

use std::{collections::VecDeque, ops::RangeBounds, cmp::Ordering};



pub struct HashDeque(VecDeque<Hash>);

impl HashDeque {
    pub fn new() -> Self {
        Self(VecDeque::from([Hash::new(0)]))
    }
    
    pub fn len(&self) -> usize { self.0.len()-1 }
    
    /// # Panics
    /// 
    /// - if not `v < MOD = 2^61 - 1`
    pub fn push_front(&mut self, v: u64) {
        self.0.push_front(self.0.front().unwrap().push_inv(v));
    }
    
    /// # Panics
    /// 
    /// - if not `v < MOD = 2^61 - 1`
    pub fn push_back(&mut self, v: u64) {
        self.0.push_back(self.0.back().unwrap().push(v));
    }
    
    pub fn pop_front(&mut self) -> Option<u64> {
        if self.0.len() == 1 { return None; }
        let mut t = self.0.pop_front().unwrap();
        t = *self.0.front().unwrap() - (t << 1);
        debug_assert!(t.0[0] == t.0[1]);
        Some(t.0[0])
    }
    
    pub fn pop_back(&mut self) -> Option<u64> {
        if self.0.len() == 1 { return None; }
        let t = self.0.pop_back().unwrap() - (*self.0.back().unwrap() << 1);
        debug_assert!(t.0[0] == t.0[1]);
        Some(t.0[0])
    }
    
    pub fn slice<'a>(&'a self, range: impl RangeBounds<usize>) -> HashSlice<'a> {
        let [l, r] = to_bounds(range, self.len());
        HashSlice { ptr: self, l, r }
    }
    
    /// `deq[range]` のハッシュを返す。`O(log(len))`
    pub fn fold(&self, range: impl RangeBounds<usize>) -> Hash {
        let [l, r] = to_bounds(range, self.len());
        self.0[r] - (self.0[l] << r-l)
    }
}



#[derive(Clone, Copy)]
pub struct HashSlice<'a> {
    ptr: &'a HashDeque,
    l: usize,
    r: usize
}

impl HashSlice<'_> {
    pub fn len(&self) -> usize { self.r - self.l }
    
    pub fn prefix(&self, len: usize) -> Hash {
        self.ptr.0[self.l+len] - (self.ptr.0[self.l] << len)
    }
    
    pub fn lcp(&self, other: &Self) -> usize {
        let (mut ok, mut ng) = (0, self.len().min(other.len())+1);
        while 1 < ng-ok {
            let x = (ok+ng)/2;
            *(if self.prefix(x) == other.prefix(x) { &mut ok } else { &mut ng }) = x;
        }
        ok
    }
}

impl PartialEq for HashSlice<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.prefix(self.len()) == self.prefix(other.len())
    }
}

impl Eq for HashSlice<'_> {}

impl Ord for HashSlice<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        let lcp = self.lcp(other);
        if self.len() == lcp || other.len() == lcp {
            self.len().cmp(&other.len())
        } else {
            (other.prefix(lcp+1)-self.prefix(lcp+1))[0].cmp(&(MOD as u64/2))
        }
    }
}

impl PartialOrd for HashSlice<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
