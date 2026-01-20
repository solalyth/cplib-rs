pub use std::{
    collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet, BinaryHeap},
    cmp::{Ordering, Reverse},
    mem::{replace, take}
};

pub use crate::cplib::{
    *,
    SUBMISSION,
    ds::{unionfind::UnionFind, segtree::*, csr::CSR},
    algo::func::*,
    math::modtable::O,
    util::{output::{out, end}, traits::*, debug::epr_table},
};
