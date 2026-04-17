// #[allow(unused_imports)]
pub(crate) use crate as cplib;

pub const SUBMISSION: bool = false;



pub mod prelude;


pub mod algo {
    pub mod rolling_hash;
    pub mod bellman_ford;
    pub mod warshall_floyd;
    
    pub mod func;
    pub mod segtree_func;
    
    // pub mod seq;
}


pub mod const_fp {
    pub mod fp;
}


pub mod ds {
    pub mod unionfind;
    pub mod csr;
    pub mod segtree;
    
    pub mod trie;
    pub mod binary_trie;
    pub mod persistent_segtree;
    
    pub mod foldable_deque;
    pub mod splay_tree;
    pub mod sparse_segtree;
    pub mod convex_hull_trick_deque;
    pub mod convex_hull_trick_set;
    
    
    pub mod rolling_hash_deque;
}


pub mod geo {
    pub mod vector_i64;
}


pub mod graph {
    pub mod scc;
    pub mod tree;
    pub mod centroid;
}


pub mod math {
    pub mod lpf_sieve;
    pub mod func;
    pub mod quotient_list;
    pub mod modtable;
    pub mod xor_convolution;
}


pub mod mod998 {
    pub mod fp;
    pub mod fps;
}



pub mod traits {
    pub mod grid;
    pub mod char_util;
    pub mod iter_util;
    pub mod map_init;
    pub mod vec_split;
}


pub mod util {
    pub mod input;
    pub mod output;
    pub mod macros;
    pub mod func;
    pub mod debug;
    
    pub mod global;
    pub mod time;
}
