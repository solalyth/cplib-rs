use crate::cplib::ds::csr::Edge;

/// 木の pre-order と Euler Tour を計算する構造体。Heavy-Light Decomposition を同時に行う。
/// 
/// Euler Tour は仮想頂点との辺 `0 <-> par(0)` を考慮するため長さ `2n` である。Pre-order traversal は長さ `n` である。
/// 
/// # 木の頂点
pub struct Tree {
    edge: Edge,
    root: usize,
    
    par: Vec<usize>,
    size: Vec<usize>,
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
    pub fn new(e: &Edge, root: usize) -> Self {
        let n = e.idx_len();
        assert!(e.dat_len() == (n-1)*2);
        
        let (mut par, mut depth, mut euler, mut euler_inv, mut pre, mut pre_inv, mut head, mut next, mut size)
            = (vec![!0; n], vec![0; n], vec![0; 2*n], vec![], vec![0; 2*n], vec![], vec![!0; n], vec![!0; n], vec![1; n]);
        
        let mut dfs = vec![root];
        
        while let Some(i) = dfs.pop() {
            if i>>63 == 0 {
                for &j in &e[i] {
                    if par[i] != j {
                        par[j] = i;
                        depth[j] = depth[i]+1;
                        dfs.extend([!j, j]);
                    }
                }
            } else {
                size[par[!i]] += size[!i];
            }
        }
        
        let mut edge = Edge::new();
        
        for i in 0..n {
            edge.next_vec();
            for &e in &e[i] {
                if e != par[i] {
                    edge.push(e);
                    let j = edge[i].len()-1;
                    if size[edge[i][0]] < size[edge[i][j]] { edge[i].swap(0, j); }
                }
            }
        }
        
        dfs.extend([!root, !0, root]);
        
        while let Some(i) = dfs.pop() {
            if i>>63 == 0 {
                let pi = pre_inv.len();
                let xpi = dfs.pop().unwrap();
                if head[pi] == !0 { head[pi] = pi; next[pi] = xpi; }
                euler_inv.push(i);
                pre_inv.push(i);
                for &j in edge[i].iter().rev() {
                    dfs.extend([!j, pi, j]);
                }
                if !edge[i].is_empty() {
                    head[pi+1] = head[pi];
                    next[pi+1] = next[pi];
                }
            } else {
                pre[n+!i] = pre_inv.len();
                euler_inv.push(n+!i);
            }
        }
        
        
        for i in 0..2*n { euler[euler_inv[i]] = i; }
        for i in 0..n { pre[pre_inv[i]] = i; }
        
        // crate::epr!("head = {head:?}\nnext = {next:?}");
        
        Self { edge, root, par, size, depth, head, euler, euler_inv, pre, pre_inv, next }
    }
    
    pub fn root(&self) -> usize { self.root }
    pub fn len(&self) -> usize { self.edge.idx_len() }
    
    /// `par(root) == !0`
    pub fn par(&self, i: usize) -> usize { self.par[i] }
    
    // /// `par_edge(i)` は下向き、`par_edge(n+i)` は上向きの辺の index を表す。`par_edge(root) == par_edge(n+root) == !0`
    // pub fn par_edge(&self, i: usize) -> (usize, usize) { self.edge[i][0] }
    
    /// `depth[root] == 0`
    pub fn depth(&self, i: usize) -> usize { self.depth[i] }
    pub fn size(&self, i: usize) -> usize { self.size[i] }
    
    /// heavy edge を返す。
    pub fn heavy(&self, i: usize) -> Option<usize> {
        self.edge[i].first().copied()
    }
    /// light edge を返す。
    pub fn light(&self, i: usize) -> &[usize] {
        if self.edge[i].is_empty() { &[] } else { &self[i][1..] }
    }
    pub fn subtree_pre(&self, i: usize) -> &[usize] {
        &self.pre_inv[self.pre[i]..self.pre[i]+self.size[i]]
    }
    
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
    
    /// `u -> root` パスを pidx の区間の列で表現する。
    /// `closed = true` のとき閉区間 `[L, R]` で表現し、特に `0` を含みうる。
    /// `closed = false` のとき左半開区間 `(L, R]` の列で表現し、特に `0` を含まない。
    /// 
    /// 上向きのパスであるから、積を取るときは `up(R) * up(R-1) * ... * up(L+1)` の方向になることに注意。
    pub fn path_root(&self, mut pu: usize, closed: bool) -> Vec<(usize, usize)> {
        let mut res = vec![];
        while pu != !0 {
            res.push((if closed {self.head[pu]} else {self.head[pu].max(1)-1}, pu));
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
        let mut path = [self.path_root(pu, false), self.path_root(pv, false)];
        while let (Some(&(ul, ur)), Some(&(vl, vr))) = (path[0].last(), path[1].last()) {
            if ul != vl { break; }
            path[0].pop();
            path[1].pop();
            if ur < vr { path[1].push((vl, vr)); break; }
            if vr < ur { path[0].push((ul, ur)); break; }
        }
        path
    }
    
    /// `pu -> root` パスの `k` 個目の頂点を `Ok(pidx)` を返す。存在しないとき、`Err(k - depth)` を返す。
    pub fn kth_ancestor(&self, mut pu: usize, mut k: usize) -> Result<usize, usize> {
        while pu != !0 {
            let pl = self.head[pu].max(1)-1;
            if k < pu-pl { return Ok(pu-k); }
            k -= pu-pl;
            pu = self.next[pu];
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
    
    /// 重心 `c` を根としたとき、部分木のサイズは `N-1` の切り上げ半分以下となる。
    /// 重心が二個あるケースを考慮すると、部分木のサイズは (重心以外の頂点数) の切り下げ半分以下となる。
    pub fn centroid(&self) -> usize {
        let mut r = self.root;
        'q: loop {
            for &i in &self.edge[r] {
                if self.size[i] >= (self.len()+1)/2 { r = i; continue 'q; }
            }
            break;
        }
        r
    }
    
    pub fn debug_edge(&self) {
        for _i in 0..self.len() {
            crate::epr!("edge[{_i}] = {:?}", self[_i].iter().collect::<Vec<_>>());
        }
    }
}

impl std::ops::Index<usize> for Tree {
    type Output = [usize];
    fn index(&self, i: usize) -> &Self::Output { &self.edge[i] }
}
