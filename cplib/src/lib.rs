// #[allow(unused_imports)]
pub(crate) use crate as cplib;

pub const LOCAL: bool = true;



pub mod prelude;


pub mod algo {
    pub mod rolling_hash;
    pub mod lcs;
    
    pub mod func;
    
    // pub mod seq;
}


pub mod const_fp {
    pub mod fp;
    pub mod fps998;
}


pub mod ds {
    pub mod unionfind;
    pub mod csr;
    pub mod segtree;
    
    pub mod trie;
    pub mod binary_trie;
    pub mod persistent_segtree;
    
    // pub mod foldable_deque;
    // pub mod splay_tree_core;
    // pub mod sparse_segtree;
    // pub mod convex_hull_trick_deque;
    // pub mod convex_hull_trick_set;
    // pub mod disjoint_sparse_table;
    
    // pub mod wavelet_matrix;
    
    // pub mod rolling_hash_deque;
    // pub mod priority_queue;
}


pub mod geo {
    pub mod vector_i64;
}


pub mod graph {
    pub mod scc;
    pub mod tree;
    pub mod centroid;
    pub mod cartesian_tree;
    
}


pub mod math {
    pub mod lpf_sieve;
    pub mod func;
    pub mod modtable;
    
    pub mod quotient_list;
    pub mod xor_convolution;
    pub mod minplus_convolution;
    
    pub mod digit_decomp;
}


pub mod misc {
}


pub mod traits {
    pub mod prelude {
        pub use super::{
            grid::*,
            char_util::*,
            iter_util::*,
            int_util::*,
            vec_util::*,
            vec_split::*,
        };
    }
    
    pub mod grid;
    pub mod char_util;
    pub mod iter_util;
    pub mod int_util;
    pub mod vec_util;
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
    
    pub mod order;
}
