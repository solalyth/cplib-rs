// #[allow(unused_imports)]
pub(crate) use crate as cplib;

pub const SUBMISSION: bool = false;



pub mod prelude;

pub mod algo {
    pub mod rolling_hash;
    pub mod bellman_ford;
    pub mod warshall_floyd;
    pub mod lcs;
    
    pub mod func;
    pub mod segtree_func;
    
    // pub mod seq;
}

pub mod ds {
    pub mod unionfind;
    pub mod foldable_deque;
    pub mod splay_tree;
    pub mod segtree;
    pub mod sparse_segtree;
    pub mod convex_hull_trick;
    pub mod trie;
    pub mod csr;
    
    // pub mod pointer_segtree;
    
    pub mod ordered;
    pub mod sorted_set;
    
    pub mod rolling_hash_deque;
    
    // pub mod light_splay_tree;
    // pub mod super_splay_tree;
    
    // pub mod rangeset;
    // pub mod multiset;
}


pub mod graph {
    pub mod scc;
    pub mod functional_graph;
    pub mod tree;
    
    // pub mod old_tree;
    pub mod centroid;
}

pub mod math {
    pub mod sieve;
    pub mod montgomery;
    // pub mod barrett;
    
    pub mod func;
    pub mod quotient_list;
    pub mod modtable;
    
    pub mod vector_i64;
    
    pub mod xor_convolution;
    
    // pub mod mod998;
    // pub mod matrix; // todo
    
    // pub mod vector;
}

pub mod mod998 {
    pub mod fp;
    pub mod fps;
}

pub mod util {
    pub mod input;
    pub mod output;
    pub mod macros;
    pub mod traits;
    pub mod func;
    pub mod debug;
    
    pub mod global;
    pub mod time;
}
