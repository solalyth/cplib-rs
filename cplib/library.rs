mod external {
    pub use {
        // atcoder, yukicoder
        proconio::{
            input, input_interactive,
            marker::{Chars as chars, Usize1 as usize1}
        },
        
        // atcoder only
        itertools::{Itertools, iproduct},
        superslice::Ext,
        num_integer::{gcd, lcm, Roots},
        // num_bigint::BigUint,
        ac_library,
        // rand
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
        
        
        // import!("ds/sorted_set");
        // import!("ds/trie");
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
        import!("math/sieve");
        import!("math/modtable");
        
        // import!("math/xor_convolution");
        // import!("math/vector_i64");
        // import!("math/montgomery");
    }
    
    pub mod mod998 {
        // import!("mod998/fp");
        // import!("mod998/fps");
    }
    
    pub mod util {
        import!("util/output");
        import!("util/traits");
        import!("util/macros");
        import!("util/func");
        import!("util/debug");
        // import!("util/global");
        
        
        // import!("util/time");
    }
}
