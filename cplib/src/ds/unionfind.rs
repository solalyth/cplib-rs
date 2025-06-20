pub use crate::cplib::abstracts::{Group, Nop};
use crate::cplib::util::func::join;

/// Potential 付き Union Find (union by size, path compression)
/// 
/// Potential として可換群が乗る。
/// 
/// # 参考
/// 
/// - https://37zigen.com/union-find
/// 
/// # 搭載機能
/// 
/// - `Clone`, `Debug`
/// 
/// # 例題
/// 
/// - https://atcoder.jp/contests/abc328/tasks/abc328_f
pub struct UnionFind<Op: Group> {
    par: Vec<usize>,
    size: Vec<usize>,
    diff: Vec<Op::T>,
    next: Vec<usize>
}



impl UnionFind<Nop> {
    pub fn new_nop(len: usize) -> Self { Self::new(len) }
}

impl<Op: Group> UnionFind<Op> {
    pub fn new(len: usize) -> Self {
        UnionFind { par: (0..len).collect(), size: vec![1; len], diff: vec![Op::e(); len], next: (0..len).collect() }
    }
    
    pub fn extend(&mut self, len: usize) {
        let bef = self.len();
        self.par.extend(bef..len);
        self.size.resize(len, 1);
        self.diff.resize(len, Op::e());
        self.next.extend(bef..len);
    }
    
    pub fn len(&self) -> usize { self.par.len() }
    
    pub fn leader(&mut self, i: usize) -> usize {
        let p = self.par[i];
        if self.par[p] == p { return p; }
        let u = self.leader(p);
        self.diff[i] = Op::add(&self.diff[i], &self.diff[p]);
        self.par[i] = u;
        u
    }
    
    pub fn size(&mut self, mut i: usize) -> usize { i = self.leader(i); self.size[i] }
    pub fn is_same(&mut self, i: usize, j: usize) -> bool { self.leader(i) == self.leader(j) }
    
    /// `potential[i] - potential[j]` を返す。
    /// 
    /// # Panics
    /// 
    /// if not `is_same(i, j)`
    pub fn diff(&mut self, i: usize, j: usize) -> Op::T {
        assert!(self.is_same(i, j));
        Op::sub(&self.diff[i], &self.diff[j])
    }
    
    /// `potential[i] - potential[j] = w` となるよう情報を追加する。
    /// 整合性を保てないとき `None` を返す。そうでないとき `Some((new_leader, old_leader))` を返す。ただし、同じ親であるとき `new_leader == old_leader` である。
    pub fn merge(&mut self, i: usize, j: usize, mut w: Op::T) -> Option<(usize, usize)> {
        let (mut u, mut v) = (self.leader(i), self.leader(j));
        w = Op::sub(&Op::add(&w, &self.diff[j]), &self.diff[i]);
        if u == v { return if w == Op::e() { Some((u, u)) } else { None } }
        if !(self.size[u] < self.size[v]) { (u, v) = (v, u); w = Op::inv(&w); }
        self.par[u] = v;
        self.diff[u] = w;
        self.size[v] += self.size[u];
        self.next.swap(i, j);
        Some((v, u))
    }
    
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut res = crate::nest![void; self.len()];
        for i in 0..self.len() { res[self.leader(i)].push(i); }
        res.retain(|v| v.len() != 0);
        res
    }
    
    pub fn group(&self, i: usize) -> Vec<usize> {
        let (mut res, mut j) = (vec![i], self.par[i]);
        while j != i { res.push(j); j = self.par[j]; }
        res
    }
    
    pub fn leaders(&self) -> Vec<usize> {
        (0..self.len()).filter(|&i| self.par[i] == i).collect()
    }
}

impl<Op: Group> Clone for UnionFind<Op> {
    fn clone(&self) -> Self {
        Self { par: self.par.clone(), size: self.size.clone(), diff: self.diff.clone(), next: self.next.clone() }
    }
}

impl<Op: Group> std::fmt::Debug for UnionFind<Op> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut uf = self.clone();
        let g = uf.groups().into_iter().map(|s| {
            join(s.into_iter().map(|i| format!("{i}({:?})", uf.diff[i]).trim_end_matches("(())").into())).unwrap()
        });
        write!(f, "[{}]", join(g.into_iter().map(|s| format!("{{{s}}}"))).unwrap_or(String::new()))
    }
}
