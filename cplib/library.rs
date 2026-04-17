mod external {
    pub use {
        // atcoder, yukicoder
        proconio::{
            input, input_interactive,
            marker::{Chars as chars, Usize1 as usize1}
        },
        itertools::{Itertools, iproduct},
        num::integer::{gcd, lcm, Roots},
        
        // atcoder only
        // ac_library,
        // superslice::Ext,
        // rand,
        // num_bigint::BigUint,
    };
}

mod cplib {
    #![allow(unused_macros, dead_code)]
    
    pub const SUBMISSION: bool = true;
    
    import!("prelude");
    
    pub mod ds {
        import!("ds/unionfind");
        import!("ds/segtree");
        import!("ds/csr");
        
        // import!("ds/persistent_segtree");
        
        
        // import!("ds/trie");
        // import!("ds/binary_trie");
        // import!("ds/sorted_set");
        // import!("ds/foldable_deque");
        // import!("ds/splay_tree");
        // import!("ds/sparse_segtree");
        // import!("ds/rolling_hash_deque");
        
        // import!("ds/ordered");
        
        // import!("ds/multiset");
        // import!("ds/convex_hull_trick");
    }
    
    pub mod algo {
        import!("algo/func");
        
        // import!("algo/rolling_hash");
        
        // import!("algo/bellman_ford");
        // import!("algo/warshall_floyd");
        // import!("algo/lcs");
    }
    
    pub mod graph {
        import!("graph/tree");
        
        // import!("graph/scc");
        // import!("graph/centroid");
        // import!("graph/functional");
        // import!("graph/old_tree");
    }
    
    pub mod math {
        import!("math/func");
        import!("math/lpf_sieve");
        import!("math/modtable");
        
        // import!("math/quotient_list");
        
        // import!("math/xor_convolution");
        // import!("math/vector_i64");
        // import!("math/montgomery");
    }
    
    pub mod mod998 {
        // import!("mod998/fp");
        // import!("mod998/fps");
    }
    
    pub mod traits {
        import!("traits/grid");
        import!("traits/char_util");
        import!("traits/iter_util");
        import!("traits/map_init");
        import!("traits/vec_split");
    }
    
    pub mod util {
        import!("util/output");
        import!("util/macros");
        import!("util/func");
        import!("util/debug");
        // import!("util/global");
        
        // import!("util/input");
        
        // import!("util/time");
    }
}
