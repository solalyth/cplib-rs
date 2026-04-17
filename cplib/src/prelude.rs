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
    traits::{grid::Grid, char_util::CharUtil, map_init::MapInit, vec_split::VecSplit, iter_util::IterUtil},
    util::{output::{out, end}, debug::epr_table},
};
