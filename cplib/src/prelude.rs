pub use std::{
    collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet, BinaryHeap},
    cmp::{Ordering, Reverse},
    mem::{swap, replace}
};

pub use crate::cplib::{
    *,
    SUBMISSION,
    ds::segtree::SegtreeOp,
    util::{
        output::{out, end}, traits::*, func::binary_search
    },
};



use std::fmt;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct F64(pub f64);
impl Eq for F64 {}
impl Ord for F64 { fn cmp(&self, other: &Self) -> Ordering { self.partial_cmp(other).unwrap() } }
impl fmt::Debug for F64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
