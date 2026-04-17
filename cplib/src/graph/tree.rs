use crate::cplib::ds::csr::Edge;

/// 木の pre-order と Euler Tour を計算する構造体。Heavy-Light Decomposition を同時に行う。
/// 
/// # 木の頂点
pub struct Tree {
    root: usize,
    
    par: Vec<usize>,
    par_edge: Vec<usize>,
    depth: Vec<usize>,
    
    /// idx -> eidx
    euler: Vec<usize>,
    /// eidx -> idx
    euler_inv: Vec<usize>,
    
    /// idx -> pidx
    pre: Vec<usize>,
    /// pidx -> idx
    pre_inv: Vec<usize>,
    
    /// pidx -> pidx of head vertex
    head: Vec<usize>,
    /// pidx -> pidx of next vertex (= parent of head)
    next: Vec<usize>,
}

impl Tree {
    pub fn new(edge: &mut Edge, root: usize) -> Self {
        let n = edge.idx_len();
        
        assert!(edge.dat_len() == (n-1)*2);
        
        let (mut par, mut par_edge, mut depth, mut euler, mut euler_inv, mut pre, mut pre_inv, mut head, mut next, mut size)
            = (vec![!0; n], vec![!0; 2*n], vec![0; n], vec![0; 2*n], vec![], vec![0; n], vec![], vec![!0; n], vec![!0; n], vec![1; n]);
        
        let mut dfs = vec![(3, root, 0), (2, root, !0), (0, root, root)];
        
        while let Some((f, i, x)) = dfs.pop() {
            if f == 0 {
                // size dfs in
                // x: parent(i) == x
                for l in (0..edge[i].len()).rev() {
                    let (j, k) = edge[i][l];
                    if j == x {
                        par_edge[n+i] = k;
                    } else {
                        par[j] = i;
                        par_edge[j] = k;
                        depth[j] = depth[i]+1;
                        dfs.push((1, j, l)); dfs.push((0, j, i));
                    }
                }
            } else if f == 1 {
                // size dfs out
                // x: edge[par[i]][x] == j
                size[par[i]] += size[i];
                let t = edge[par[i]][0].0;
                if t != par[par[i]] && size[t] < size[i] {
                    edge[par[i]].swap(0, x);
                }
            } else if f == 2 {
                // euler dfs in
                // x: pre[par[i]] == x
                let pi = pre_inv.len();
                if head[pi] == !0 { head[pi] = pi; next[pi] = x; }
                euler_inv.push(i); pre_inv.push(i);
                for &(j, _) in edge[i].iter().rev() {
                    if j == par[i] { continue; }
                    dfs.push((3, j, 0)); dfs.push((2, j, pi));
                }
                if edge[i][0].0 != par[i] { head[pi+1] = head[pi]; next[pi+1] = next[pi]; }
            } else {
                // euler dfs out
                euler_inv.push(n+i);
            }
        }
        
        for i in 0..2*n { euler[euler_inv[i]] = i; }
        for i in 0..n { pre[pre_inv[i]] = i; }
        
        Self { root, par, par_edge, depth, head, euler, euler_inv, pre, pre_inv, next }
    }
    
    pub fn len(&self) -> usize { self.par.len() }
    
    pub fn lca_p(&self, mut pu: usize, mut pv: usize) -> usize {
        loop {
            if !(pu < pv) { (pu, pv) = (pv, pu); }
            if self.head[pu] == self.head[pv] { return pu; }
            pv = self.next[pv];
        }
    }
    
    pub fn lca(&self, u: usize, v: usize) -> usize {
        self.pre_inv[self.lca_p(self.pre[u], self.pre[v])]
    }
    
    /// `u -> root` パスを pidx の左半開区間 `(L, R]` の列で表現する。
    /// 
    /// 上向きのパスであるから、積を取るときは `up(R) * up(R-1) * ... * up(L+1)` の方向になることに注意。
    pub fn path_root(&self, mut pu: usize) -> Vec<(usize, usize)> {
        let mut res = vec![];
        while pu != 0 {
            res.push((self.head[pu].max(1)-1, pu));
            pu = self.next[pu];
        }
        res
    }
    
    /// `u -> v` パスを pidx の左半開区間 `(L, R]` の列で表現する。
    /// 
    /// 正確に述べると、`u -> lca(u, v)` と `v -> lca(u, v)` のパスをそれぞれ返す。
    /// 
    /// `res[0]` は上向きのパスであり、積を取るときは `up(R) * up(R-1) * ... * up(L+1)` の方向になることに注意。
    /// また、`LCA(u, v)` に対応する辺は含まれないため、頂点属性の積を取るときは注意。
    pub fn path(&self, pu: usize, pv: usize) -> [Vec<(usize, usize)>; 2] {
        let mut path = [self.path_root(pu), self.path_root(pv)];
        while let Some((ul, ur)) = path[0].pop() {
            let Some((vl, vr)) = path[1].pop() else { break; };
            if ul != vl { break; }
            if ur < vr { path[1].push((vl, vr)); }
            if vr < ur { path[0].push((ul, ur)); }
        }
        path
    }
    
    /// `pu -> root` パスの `k` 個目の頂点を `Ok(pidx)` を返す。存在しないとき、`Err(k - depth)` を返す。
    pub fn kth_ancestor_root(&self, mut pu: usize, mut k: usize) -> Result<usize, usize> {
        while pu != 0 {
            let pl = self.head[pu].max(1)-1;
            if k < pu-pl { return Ok(pu-k); }
            pu = self.next[pu];
            k -= pu-pl;
        }
        if k == 0 { Ok(0) } else { Err(k) }
        
        // you can write with `path_root` like this:
        
        // let path = self.path_root(pu);
        // for (l, r) in path {
        //     if k < r-l {
        //         return Ok(r-k);
        //     }
        // }
        // if k == 0 { Ok(self.root) } else { Err(k) }
    }
    
    pub fn root(&self) -> usize { self.root }
    
    /// `par(root) == !0`
    pub fn par(&self, i: usize) -> usize { self.par[i] }
    /// `par_edge(i)` は下向き、`par_edge(n+i)` は上向きの辺の index を表す。`par_edge(root) == par_edge(n+root) == !0`
    pub fn par_edge(&self, i: usize) -> usize { self.par_edge[i] }
    /// `depth[root] == 0`
    pub fn depth(&self, i: usize) -> usize { self.depth[i] }
    
    /// idx -> pidx
    pub fn pre(&self, i: usize) -> usize { self.pre[i] }
    /// pidx -> idx
    pub fn pre_inv(&self, i: usize) -> usize { self.pre_inv[i] }
    /// idx -> eidx
    pub fn euler(&self, i: usize) -> usize { self.euler[i] }
    /// eidx -> idx
    pub fn euler_inv(&self, i: usize) -> usize { self.euler_inv[i] }
    
    pub fn pre_order(&self) -> &[usize] { &self.pre_inv }
    pub fn euler_order(&self) -> &[usize] { &self.euler_inv }
}

// #![allow(dead_code)]

// pub use crate::cplib::ds::csr::Edge;
// use crate::cplib::ds::segtree::{Segtree, SegtreeOp};
// use std::cell::UnsafeCell;

// const MASK: usize = (1<<32)-1;

// pub struct Tree<'a> {
//     edge: &'a Edge,
//     root: usize,
//     par: Vec<usize>,
//     depth: Vec<usize>, // depth[root] = 0
//     lca: UnsafeCell<Segtree<LCA>>, // ET-order
//     /// vertex-idx -> ET-idx
//     euler: Vec<usize>,
//     /// ET-idx -> vertex-idx
//     euler_inv: Vec<usize>,
// }

// impl<'a> Tree<'a> {
//     pub fn new(edge: &'a Edge, root: usize) -> Self {
//         let n = edge.idx_len();
//         let mut par = vec![root; n];
//         let mut euler = vec![0; n*2];
//         let mut euler_inv = vec![];
//         let mut lca = vec![];
//         let mut depth = vec![0; n];
//         let mut dfs = vec![root+n, root];
        
//         assert!(edge.dat_len() == (n-1)*2);
        
//         while let Some(i) = dfs.pop() {
//             euler[i] = euler_inv.len();
//             euler_inv.push(i);
//             if i < n {
//                 lca.push((depth[i]+1<<32)+i);
//                 for &(j, _) in edge[i].iter().rev() {
//                     if par[i] != j {
//                         par[j] = i;
//                         depth[j] = depth[i]+1;
//                         dfs.push(j+n);
//                         dfs.push(j);
//                     }
//                 }
//             } else {
//                 lca.push((depth[i-n]<<32)+par[i-n]);
//             }
//         }
//         par[root] = !0;
//         Self { edge, root, par, depth, lca: UnsafeCell::new(Segtree::from_iter(lca)), euler, euler_inv }
//     }
    
//     fn len(&self) -> usize { self.edge.idx_len() }
//     pub fn par(&self, i: usize) -> usize { self.par[i] }
//     pub fn depth(&self, i: usize) -> usize { self.depth[i] }
//     pub fn lca(&self, u: usize, v: usize) -> usize {
//         let (eu, ev) = crate::minmax!(self.euler[u], self.euler[v]);
//         unsafe{&mut *self.lca.get()}.fold(eu..=ev) & MASK
//     }
//     pub fn kth_ancestor(&self, u: usize, k: usize) -> Option<usize> {
//         let n = self.len();
//         let r = unsafe{&mut *self.lca.get()}.max_right(self.euler[u], 2*n, |&(mut v)| {
//             v &= MASK;
//             v < 2*n && self.depth[u] <= self.depth[v]+k
//         });
//         if r == 2*n+1 { None } else { Some(self.euler_inv[r-1]-n) }
//     }
//     pub fn dist(&self, u: usize, v: usize) -> usize {
//         let p = self.lca(u, v);
//         self.depth[u] + self.depth[v] - 2*self.depth[p]
//     }
//     /// ET-order を返す。`in: i, out: i+N`
//     pub fn order(&self) -> &[usize] { &self.euler_inv }
//     /// `(max depth, idx)`
//     pub fn depth_max(&self) -> (usize, usize) { (0..self.edge.idx_len()).map(|i| (self.depth[i], i)).max().unwrap() }
// }


// struct LCA;
// impl SegtreeOp for LCA {
//     type Value = usize;
//     type Lazy = ();
//     fn id_value() -> Self::Value { !0 }
//     fn prod_value(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value { *lhs.min(rhs) }
// }
