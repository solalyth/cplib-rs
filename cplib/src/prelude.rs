pub use std::{
    collections::{VecDeque, HashMap, HashSet, BTreeMap, BTreeSet, BinaryHeap},
    cmp::{Ordering, Reverse},
    mem::{swap, replace}
};

pub use crate::cplib::{
    ds::segtree::SegtreeOp,
    util::{
        output::{out, end, EndFlag}, traits::*, func::binary_search
    }
};
