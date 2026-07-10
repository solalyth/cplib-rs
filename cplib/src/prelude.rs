pub use std::{
    collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet, BinaryHeap},
    cmp::{Ordering, Reverse},
    mem::{replace, take}
};

pub use crate::cplib::{
    *,
    LOCAL,
    ds::{segtree::*, csr::{CSR, Edge}},
    algo::func::*,
    traits::prelude::*,
    util::output::{out, end},
};
