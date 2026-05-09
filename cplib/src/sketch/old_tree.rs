#![allow(dead_code)]

pub use crate::cplib::ds::csr::Edge;
use crate::cplib::ds::segtree::{Segtree, SegtreeOp};
use std::cell::UnsafeCell;

const MASK: usize = (1<<32)-1;

pub struct Tree<'a> {
    edge: &'a Edge,
    root: usize,
    par: Vec<usize>,
    depth: Vec<usize>, // depth[root] = 0
    lca: UnsafeCell<Segtree<LCA>>, // ET-order
    /// vertex-idx -> ET-idx
    euler: Vec<usize>,
    /// ET-idx -> vertex-idx
    euler_inv: Vec<usize>,
}

impl<'a> Tree<'a> {
    pub fn new(edge: &'a Edge, root: usize) -> Self {
        let n = edge.idx_len();
        let mut par = vec![root; n];
        let mut euler = vec![0; n*2];
        let mut euler_inv = vec![];
        let mut lca = vec![];
        let mut depth = vec![0; n];
        let mut dfs = vec![root+n, root];
        
        assert!(edge.dat_len() == (n-1)*2);
        
        while let Some(i) = dfs.pop() {
            euler[i] = euler_inv.len();
            euler_inv.push(i);
            if i < n {
                lca.push((depth[i]+1<<32)+i);
                for &(j, _) in edge[i].iter().rev() {
                    if par[i] != j {
                        par[j] = i;
                        depth[j] = depth[i]+1;
                        dfs.push(j+n);
                        dfs.push(j);
                    }
                }
            } else {
                lca.push((depth[i-n]<<32)+par[i-n]);
            }
        }
        par[root] = !0;
        Self { edge, root, par, depth, lca: UnsafeCell::new(Segtree::from_iter(lca)), euler, euler_inv }
    }
    
    fn len(&self) -> usize { self.edge.idx_len() }
    pub fn par(&self, i: usize) -> usize { self.par[i] }
    pub fn depth(&self, i: usize) -> usize { self.depth[i] }
    pub fn lca(&self, u: usize, v: usize) -> usize {
        let (eu, ev) = crate::minmax!(self.euler[u], self.euler[v]);
        unsafe{&mut *self.lca.get()}.fold(eu..=ev) & MASK
    }
    pub fn kth_ancestor(&self, u: usize, k: usize) -> Option<usize> {
        let n = self.len();
        let r = unsafe{&mut *self.lca.get()}.max_right(self.euler[u], 2*n, |&(mut v)| {
            v &= MASK;
            v < 2*n && self.depth[u] <= self.depth[v]+k
        });
        if r == 2*n+1 { None } else { Some(self.euler_inv[r-1]-n) }
    }
    pub fn dist(&self, u: usize, v: usize) -> usize {
        let p = self.lca(u, v);
        self.depth[u] + self.depth[v] - 2*self.depth[p]
    }
    /// ET-order を返す。`in: i, out: i+N`
    pub fn order(&self) -> &[usize] { &self.euler_inv }
    /// `(max depth, idx)`
    pub fn depth_max(&self) -> (usize, usize) { (0..self.edge.idx_len()).map(|i| (self.depth[i], i)).max().unwrap() }
}


struct LCA;
impl SegtreeOp for LCA {
    type Value = usize;
    type Lazy = ();
    fn id_value() -> Self::Value { !0 }
    fn prod_value(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value { *lhs.min(rhs) }
}
