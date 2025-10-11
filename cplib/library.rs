mod cplib {
    #![allow(unused_macros, dead_code)]
    
    pub const SUBMISSION: bool = true;
    
    import!("prelude");
    
    pub mod ds {
        import!("ds/unionfind");
        // import!("ds/foldable_deque");
        import!("ds/segtree");
        // import!("ds/splay_tree");
        // import!("ds/sparse_segtree");
        import!("ds/trie");
        
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
        // import!("graph/tree");
        // import!("graph/scc");
        // import!("graph/functional");
    }
    
    import!("abstracts");
    
    pub mod math {
        import!("math/func");
        import!("math/prime");
        // import!("math/modtable");
        // import!("math/montgomery");
    }
    
    pub mod util {
        import!("util/output");
        import!("util/traits");
        import!("util/macros");
        import!("util/func");
        // import!("util/time");
    }
}
