// #[allow(unused_imports)]
pub(crate) use crate as cplib;

pub const SUBMISSION: bool = false;



pub mod prelude;

pub mod ds {
    pub mod unionfind;
    pub mod foldable_deque;
    pub mod splay_tree;
    pub mod segtree;
    pub mod sparse_segtree;
    
    pub mod ordered;
    
    // pub mod rangeset;
    // pub mod multiset;
}

pub mod algo {
    pub mod rolling_hash;
    pub mod bellman_ford;
    pub mod warshall_floyd;
    pub mod lcs;
    
    pub mod func;
    
    // pub mod seq;
}

pub mod graph {
    pub mod tree;
}

pub mod abstracts;

pub mod math {
    pub mod prime;
    pub mod montgomery;
    // pub mod barrett;
    // pub mod matrix;
    
    pub mod func;
    pub mod modtable;
    
    // pub mod vector;
}

pub mod util {
    pub mod output;
    pub mod macros;
    pub mod traits;
    pub mod func;
    
    pub mod global;
}
