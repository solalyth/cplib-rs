use std::fmt::Debug;

use crate::cplib::ds::segtree::*;

pub struct Sorted<T: Ord> {
    seg: Segtree<RangeSum>,
    order: Vec<T>,
}

impl<T: Ord> Sorted<T> {
    pub fn new(mut v: Vec<T>) -> Self {
        v.sort(); v.dedup();
        let seg = RangeSum::segtree_new(v.len());
        Self { seg, order: v }
    }
    
    pub fn len(&mut self) -> i64 { self.seg.fold(..) }
    
    pub fn add(&mut self, x: &T, n: i64) {
        let idx = self.order.binary_search(x).unwrap();
        self.seg.set(idx, |e| { *e += n; });
    }
    
    pub fn get(&mut self, idx: i64) -> &T {
        let i = self.seg.max_right(0, self.order.len(), |&e| e <= idx);
        &self.order[i]
    }
}

struct RangeSum;
impl SegtreeOp for RangeSum {
    type Value = i64;
    type Lazy = ();
    fn id_value() -> Self::Value { 0 }
    fn prod_value(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value { lhs+rhs }
}

impl<T: Ord + Clone + Debug> Debug for Sorted<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = vec![];
        for i in 0..self.order.len() {
            for _ in 0..self.seg[i] {
                v.push(self.order[i].clone());
            }
        }
        write!(f, "{v:?}")
    }
}
