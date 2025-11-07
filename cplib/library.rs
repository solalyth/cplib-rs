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
        ac_library::{self, ModInt998244353 as Mint},
        rand
    };
}

mod cplib {
    #![allow(unused_macros, dead_code)]
    
    pub const SUBMISSION: bool = true;
    
    import!("prelude");
    import!("debug");
    
    pub mod ds {
        import!("ds/unionfind");
        import!("ds/segtree");
        
        // import!("ds/foldable_deque");
        // import!("ds/splay_tree");
        // import!("ds/sparse_segtree");
        
        // import!("ds/ordered");
        
        // import!("ds/multiset");
        // import!("ds/convex_hull_trick");
    }
    
    pub mod algo {
        // import!("algo/rolling_hash");
        // import!("algo/bellman_ford");
        // import!("algo/warshall_floyd");
        // import!("algo/lcs");
        import!("algo/func");
    }
    
    pub mod graph {
        import!("graph/trie");
        // import!("graph/tree");
        // import!("graph/scc");
        // import!("graph/functional");
    }
    
    pub mod math {
        import!("math/func");
        import!("math/sieve");
        // import!("math/modtable");
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
        // import!("util/time");
    }
}
