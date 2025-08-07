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
        output::{Output, out, end}, traits::*, func::binary_search
    },
};
