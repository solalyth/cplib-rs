use std::fmt::Debug;
use crate::cplib::util::func::join;

pub trait Abelian {
    type T: Clone + Eq;
    fn e() -> Self::T;
    fn add(l: &Self::T, r: &Self::T) -> Self::T;
    fn inv(x: &Self::T) -> Self::T;

    fn sub(l: &Self::T, r: &Self::T) -> Self::T { Self::add(l, &Self::inv(r)) }
}

pub struct Nop;
impl Abelian for Nop {
    type T = ();
    fn e() {}
    fn add(_: &(), _: &()) {}
    fn inv(_: &()) {}
}



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
pub struct UnionFind<Op: Abelian> {
    par: Vec<usize>,
    size: Vec<usize>,
    diff: Vec<Op::T>,
    next: Vec<usize>
}



impl UnionFind<Nop> {
    pub fn new_nop(len: usize) -> Self { Self::new(len) }
}

impl<Op: Abelian> UnionFind<Op> {
    pub fn new(len: usize) -> Self {
        UnionFind { par: (0..len).collect(), size: vec![1; len], diff: vec![Op::e(); len], next: (0..len).collect() }
    }
    
    pub fn clear(&mut self) {
        for i in 0..self.len() {
            self.par[i] = i;
            self.size[i] = 1;
            self.diff[i] = Op::e();
            self.next[i] = i;
        }
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
    
    /// `P[i] - P[j]` を返す。
    /// 
    /// # Panics
    /// 
    /// if not `is_same(i, j)`
    pub fn diff(&mut self, i: usize, j: usize) -> Op::T {
        assert!(self.is_same(i, j));
        Op::sub(&self.diff[i], &self.diff[j])
    }
    
    /// `P[i] - P[j] = w` となるよう辺を追加する。
    /// 整合性を保てないとき `None` を返す。元々連結であったとき `Some((common_leader, !0))` を返し、非連結であったとき `Some((new_leader, old_leader))` を返す。
    /// 
    /// 操作前について `size[old] <= size[new]` が保証される。
    pub fn merge(&mut self, i: usize, j: usize, mut w: Op::T) -> Option<(usize, usize)> {
        let (mut old, mut new) = (self.leader(i), self.leader(j));
        w = Op::sub(&Op::add(&w, &self.diff[j]), &self.diff[i]);
        if old == new { return if w == Op::e() { Some((old, !0)) } else { None } }
        if !(self.size[old] <= self.size[new]) { (old, new) = (new, old); w = Op::inv(&w); }
        self.par[old] = new;
        self.diff[old] = w;
        self.size[new] += self.size[old];
        self.next.swap(i, j);
        Some((new, old))
    }
    
    pub fn size_undo(&self, new: usize, old: usize) -> (usize, usize) {
        (self.size[new] - self.size[old], self.size[old])
    }
    
    /// `res[i] = { j | leader(j) == i }`
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut res = crate::nest![void; self.len()];
        for i in 0..self.len() { res[self.leader(i)].push(i); }
        res
    }
    
    pub fn group(&self, i: usize) -> Vec<usize> {
        let (mut res, mut j) = (vec![i], self.next[i]);
        while j != i { res.push(j); j = self.next[j]; }
        res
    }
    
    pub fn leaders(&self) -> Vec<usize> {
        (0..self.len()).filter(|&i| self.par[i] == i).collect()
    }
}

impl<Op: Abelian> Clone for UnionFind<Op> {
    fn clone(&self) -> Self {
        Self { par: self.par.clone(), size: self.size.clone(), diff: self.diff.clone(), next: self.next.clone() }
    }
}

impl<Op: Abelian> std::fmt::Debug for UnionFind<Op> where Op::T: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut uf = self.clone();
        let g = uf.groups().into_iter().map(|s| {
            join(s.into_iter().map(|i| format!("{i}({:?})", uf.diff[i]).trim_end_matches("(())").into())).unwrap()
        });
        write!(f, "[{}]", join(g.into_iter().map(|s| format!("{{{s}}}"))).unwrap_or(String::new()))
    }
}
