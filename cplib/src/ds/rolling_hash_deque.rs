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

use crate::cplib::{algo::rolling_hash::Hash, util::func::to_bounds};

use std::{collections::VecDeque, ops::RangeBounds};

/// 区間ハッシュを `O(log(len))` で求められる。
pub struct HashDeque(VecDeque<Hash>);

impl HashDeque {
    pub fn new() -> Self {
        Self(VecDeque::from([Hash::new(0)]))
    }
    
    pub fn len(&self) -> usize { self.0.len()-1 }
    
    /// # Panics
    /// 
    /// - if not `v < MOD = 2^61 - 1`
    pub fn push_back(&mut self, v: u64) {
        self.0.push_back(self.0.back().unwrap().push(v));
    }
    
    /// # Panics
    /// 
    /// - if not `v < MOD = 2^61 - 1`
    pub fn push_front(&mut self, v: u64) {
        self.0.push_front(self.0.front().unwrap().push_inv(v));
    }
    
    pub fn pop_back(&mut self) -> Option<u64> {
        if self.0.len() == 1 { return None; }
        let t = self.0.pop_back().unwrap() - (*self.0.back().unwrap() << 1);
        debug_assert!(t.0[0] == t.0[1]);
        Some(t.0[0])
    }
    
    pub fn pop_front(&mut self) -> Option<u64> {
        if self.0.len() == 1 { return None; }
        let mut t = self.0.pop_front().unwrap();
        t = *self.0.front().unwrap() - (t << 1);
        debug_assert!(t.0[0] == t.0[1]);
        Some(t.0[0])
    }
    
    // pub fn slice<'a>(&'a self, range: impl RangeBounds<usize>) -> HashDequeSlice<'a> {
    //     let [l, r] = to_bounds(range, self.len());
    //     HashDequeSlice { ptr: self, l, r }
    // }
    
    /// `deq[range]` のハッシュを返す。`O(log(len))`
    pub fn fold(&self, range: impl RangeBounds<usize>) -> Hash {
        let [l, r] = to_bounds(range, self.len());
        self.0[r] - (self.0[l] << r-l)
    }
}

// impl PartialEq for HashDeque {
//     fn eq(&self, other: &Self) -> bool {
//         self.slice(..).eq(&other.slice(..))
//     }
// }
// impl Ord for HashDeque {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.slice(..).cmp(&other.slice(..))
//     }
// }
// impl Eq for HashDeque {}
// impl PartialOrd for HashDeque { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.cmp(other).into() } }




// pub struct HashDequeSlice<'a> {
//     ptr: &'a HashDeque,
//     l: usize,
//     r: usize
// }

// impl<'a> HashDequeSlice<'a> {
//     fn len(&self) -> usize {
//         self.r - self.l
//     }
    
//     fn prefix(&self, len: usize) -> Hash {
//         assert!(len <= self.len());
//         self.ptr.fold(self.l..self.l+len)
//     }
// }

// impl<'a> PartialEq for HashDequeSlice<'a> {
//     fn eq(&self, other: &Self) -> bool {
//         self.len() == other.len() && self.prefix(self.len()) == other.prefix(self.len())
//     }
// }

// impl<'a> Ord for HashDequeSlice<'a> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         let (sl, tl) = (self.len(), other.len());
//         let ml = sl.min(tl);
//         if self.prefix(ml) == other.prefix(ml) { return sl.cmp(&tl); }
//         let (mut ng, mut ok) = (0, ml);
//         while 1 < ok-ng {
//             let mid = (ng+ok)/2;
//             if self.prefix(mid) == other.prefix(mid) { ng = mid; } else { ok = mid; }
//         }
//         self.ptr.deq[ok].cmp(&other.ptr.deq[ok])
//     }
// }

// impl<'a> Eq for HashDequeSlice<'a> {}
// impl<'a> PartialOrd for HashDequeSlice<'a> { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.cmp(other).into() } }
