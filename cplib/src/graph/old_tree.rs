use crate::cplib::ds::segtree::{Segtree, SegtreeOp};

/// `euler[i]`, `eular[i+n]` は辺 `par[i] -> i`, `i -> par[i]` と考えてよい。<br>
/// ただし、`par[root]` は(存在しない)頂点があると考える。このため `depth[root] = 1, depth[root+n] = 0` となっている。<br>
/// ただし、実装では `par[root] = root` となっている。これはダブリングのためである。
pub struct Tree {
    n: usize,
    e: Vec<Vec<usize>>,
    // root: usize,
    par: Vec<usize>,
    dfs_order: Vec<usize>,
    depth: Segtree<LCA>,
    euler: Vec<usize>,
    euler_inv: Vec<usize>
}

impl Tree {
    pub fn new(n: usize, e: Vec<Vec<usize>>, root: usize) -> Self {
        let mut par = vec![0; n]; par[root] = root;
        let mut depth = vec![0; 2*n];
        let (mut euler, mut euler_inv) = (vec![0; 2*n], vec![0; 2*n]);
        let (mut dfs, mut dfs_order) = (vec![root], vec![]);
        let mut d = 0;
        for cnt in 0..2*n-1 {
            let i = dfs.pop().unwrap();
            if euler[i] == 0 {
                d += 1;
                dfs_order.push(i);
                euler[i] = cnt;
                for &j in &e[i] {
                    if par[i] == j { continue; }
                    par[j] = i;
                    dfs.push(j); dfs.push(j);
                }
            } else {
                d -= 1;
                euler[n+i] = cnt;
            }
            depth[cnt] = d;
        }
        euler[root+n] = 2*n-1;
        for i in 0..2*n { euler_inv[euler[i]] = i; }
        Self { n, e, par, dfs_order, euler, euler_inv, depth: LCA::segtree_from_iter((0..2*n).map(|i| (i, depth[i]))) }
    }
    
    /// `[du, dv, dp, p]` を返す。`depth[root] = 1` であることに注意。
    pub fn lca(&mut self, u: usize, v: usize) -> [usize; 4] {
        let (eu, ev) = (self.euler[u], self.euler[v]);
        let (du, dv) = (self.depth.get(eu).1, self.depth.get(ev).1);
        let (ep, dp) = self.depth.fold(eu.min(ev)..=eu.max(ev));
        [du, dv, dp, self.euler_inv[ep]]
    }
    
    pub fn par(&self, i: usize) -> usize { self.par[i] }
    pub fn e(&self, i: usize) -> &[usize] { &self.e[i] }
    pub fn euler_in(&self, i: usize) -> usize { self.euler[i] }
    pub fn euler_out(&self, i: usize) -> usize { self.euler[i+self.n] }
    pub fn euler_inv(&self, ei: usize) -> usize { self.euler_inv[ei] }
    
    pub fn dfs_order(&self) -> &[usize] { &self.dfs_order }
}



/// (idx, depth)
pub struct LCA;
impl SegtreeOp for LCA {
    type Value = (usize, usize);
    type Lazy = ();
    fn id_value() -> Self::Value {
        (!0, !0)
    }
    fn prod_value(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        if lhs.1 <= rhs.1 { *lhs } else { *rhs }
    }
}
