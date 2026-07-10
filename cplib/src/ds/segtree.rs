//! 遅延可能 Beats! 可能セグメント木

use std::{fmt::Debug, mem::replace, ops::Index, slice::SliceIndex};

/// Operator for [`Segtree`], [`crate::ds::sparse_segtree::SparseSegtree`]
/// 
/// `apply` を使用しないなら `id_value`, `prod_value` `id_lazy` のみの実装で動く。
#[allow(unused_variables)]
pub trait SegtreeOp: Sized {
    const BEATS: bool = false;
    
    type Value: Clone;
    type Lazy: Clone;
    
    fn id_value() -> Self::Value;
    fn prod_value(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    
    fn id_lazy() -> Self::Lazy;
    fn prod_lazy(lazy: &mut Self::Lazy, ad: &Self::Lazy) {}
    
    /// `value` に `lazy` を作用させる。
    fn act(value: &mut Self::Value, lazy: &Self::Lazy) {}
    
    /// `value` に `lazy` を作用させる。成功した場合は作用させて `true` を返し、失敗した場合は何もせず `false` を返すこと。
    fn act_beats(value: &mut Self::Value, lazy: &Self::Lazy) -> bool { panic!() }
    
    
    fn segtree_new(len: usize) -> Segtree<Self> { Segtree::new(len) }
    fn segtree_from_iter(iter: impl ExactSizeIterator<Item = Self::Value>) -> Segtree<Self> {
        let mut seg = Segtree::new(iter.len());
        let len = seg.len();
        for (i, v) in iter.enumerate() { seg.tree[len+i] = v; }
        for i in (1..len).rev() { seg.update(i); }
        seg
    }
}




/// 遅延可能 Beats! 可能セグメント木
/// 
/// `lazy[i]` には `tree[i]` の子孫が受ける作用が入る。特に、`tree[i]` は `lazy[i]` が既に作用されている。
/// 
/// # 搭載機能
/// 
/// - [`Clone`], [`Debug`]
/// - [`Index`], [`IndexMut`] (遅延を解消する必要がないときのみ)
pub struct Segtree<Op: SegtreeOp> {
    pub tree: Vec<Op::Value>,
    /// `lazy[i]` = `i` の子孫が反映待ちである `Lazy`
    pub lazy: Vec<Op::Lazy>,
    dep: u32
}



impl<Op: SegtreeOp> Segtree<Op> {
    pub fn new(len: usize) -> Self {
        let dep = (len.max(2)-1).ilog2() + 2;
        Segtree { tree: vec![Op::id_value(); 1<<dep], lazy: vec![Op::id_lazy(); 1<<dep], dep }
    }
    
    /// 最下層の長さを返す。これは [`Segtree::new`] で指定した長さと異なる可能性がある。
    pub fn len(&self) -> usize { 1 << self.dep-1 }
    
    pub fn get(&mut self, mut i: usize) -> &Op::Value {
        i += self.len();
        for j in (1..self.dep).rev() { self.push(i >> j); }
        &self.tree[i]
    }
    
    pub fn set(&mut self, mut i: usize, x: Op::Value) {
        i += self.len();
        for j in (1..self.dep).rev() { self.push(i >> j); }
        self.tree[i] = x;
        for j in 1..self.dep { self.update(i >> j); }
    }
    
    pub fn set_with<T>(&mut self, mut i: usize, f: impl FnOnce(&mut Op::Value) -> T) -> T {
        i += self.len();
        for j in (1..self.dep).rev() { self.push(i >> j); }
        let res = f(&mut self.tree[i]);
        for j in 1..self.dep { self.update(i >> j); }
        res
    }
    
    pub fn push_all(&mut self) { for i in 1..self.len() { self.push(i); } }
    
    pub fn fold(&mut self, mut l: usize, mut r: usize) -> Op::Value {
        l += self.len(); r = (r+self.len()).min(self.len()*2);
        if l >= r { return Op::id_value(); }
        let (mut vl, mut vr) = (Op::id_value(), Op::id_value());
        
        for i in (1..self.dep).rev() { self.push(l >> i); self.push(r-1 >> i); }
        
        while l < r {
            if l&1 == 1 { vl = Op::prod_value(&vl, &self.tree[l]); l += 1; }
            if r&1 == 1 { vr = Op::prod_value(&self.tree[r-1], &vr); }
            l >>= 1; r >>= 1;
        }
        
        Op::prod_value(&vl, &vr)
    }
    
    pub fn apply(&mut self, mut l: usize, mut r: usize, lazy: Op::Lazy) {
        l += self.len(); r = (r+self.len()).min(self.len()*2);
        if l >= r { return; }
        for i in (1..self.dep).rev() { self.push(l >> i); self.push(r-1 >> i); }
        
        let (mut s, mut t) = (l, r);
        while s < t {
            if s&1 == 1 { self.node_apply(s, &lazy); s += 1; }
            if t&1 == 1 { t -= 1; self.node_apply(t, &lazy); }
            s >>= 1; t >>= 1;
        }
        
        for i in 1..self.dep {
            if ((l >> i) << i) != l { self.update(l >> i); }
            if ((r >> i) << i) != r { self.update(r-1 >> i); }
        }
    }
    
    /// `f(l..r) == true && f(l..r+1) == false` である `r` を一つ返す。
    /// ただし `f(l..l) == true`, `f(l..len+1) == false` であるとする。
    /// 
    /// # Panics
    /// 
    /// if not `l <= self.len`
    pub fn max_right(&mut self, l: usize, r_max: usize, f: impl Fn(&Op::Value) -> bool) -> usize {
        assert!(l <= self.len());
        if l == self.len() { return self.len().min(r_max); }
        let (mut r, mut val) = (l + self.len(), Op::id_value());
        
        for i in (1..self.dep).rev() { self.push(r >> i); }
        
        loop {
            while r&1 == 0 { r >>= 1; }
            let tmp = Op::prod_value(&val, &self.tree[r]);
            if !f(&tmp) { break; }
            val = tmp;
            r += 1;
            if r & r-1 == 0 { return self.len().min(r_max); }
        }
        
        while r < self.len() {
            self.push(r);
            r *= 2;
            let tmp = Op::prod_value(&val, &self.tree[r]);
            if f(&tmp) { val = tmp; r += 1; }
        }
        
        (r - self.len()).min(r_max)
    }
    
    /// `f(l-1..r) == false && f(l..r) == true` である `l` を 1 つ返す。
    /// ただし `f(-1..r) == false`, `f(r..r) == true` とする。
    /// 
    /// # Panics
    /// 
    /// if not `r <= self.len`
    pub fn min_left(&mut self, r: usize, f: impl Fn(&Op::Value) -> bool) -> usize {
        assert!(r <= self.len());
        if r == 0 { return 0; }
        let (mut l, mut val) = (r + self.len(), Op::id_value());
        
        for i in (1..self.dep).rev() { self.push(l-1 >> i); }
        
        loop {
            l -= 1;
            while l != 1 && l&1 == 1 { l >>= 1; }
            let tmp = Op::prod_value(&self.tree[l], &val);
            if !f(&tmp) { break; }
            val = tmp;
            if l & l-1 == 0 { return 0; }
        }
        
        while l < self.len() {
            self.push(l);
            l = 2*l + 1;
            let tmp = Op::prod_value(&self.tree[l], &val);
            if f(&tmp) { val = tmp; l -= 1; }
        }
        
        l+1 - self.len()
    }
    
    /// `tree[i]` に `lazy` を作用させ、`lazy[i]` に `lazy` を追加する。
    fn node_apply(&mut self, i: usize, lazy: &Op::Lazy) {
        Op::prod_lazy(&mut self.lazy[i], lazy);
        if Op::BEATS {
            if !Op::act_beats(&mut self.tree[i], lazy) {
                self.push(i);
                self.update(i);
            }
        } else {
            Op::act(&mut self.tree[i], lazy);
        }
    }
    
    /// `i` の子に `lazy[i]` を作用・伝搬させる。
    fn push(&mut self, i: usize) {
        debug_assert!(i < self.len());
        // Lazy が () のときは遅延いらないのでスキップ
        if std::any::type_name::<Op::Lazy>() == "()" { return; }
        
        let lazy = replace(&mut self.lazy[i], Op::id_lazy());
        self.node_apply(2*i, &lazy);
        self.node_apply(2*i+1, &lazy);
    }
    
    /// `tree[i]` を子から再計算する。
    fn update(&mut self, i: usize) {
        debug_assert!(i < self.len());
        // debug_assert!(self.lazy[i] == Op::id_lazy());
        self.tree[i] = Op::prod_value(&self.tree[2*i], &self.tree[2*i+1]);
    }
    
    // /// for debug. index(i) に対応した範囲 l..r を返す。
    // #[allow(unused)]
    // fn dbg_range(&self, i: usize) -> (usize, usize) {
    //     let d = self.depth - (i.ilog2()+1);
    //     ((i << d) - self.len(), (i+1 << d) - self.len())
    // }
}

impl<Op: SegtreeOp> Clone for Segtree<Op> {
    fn clone(&self) -> Self {
        Self { tree: self.tree.clone(), lazy: self.lazy.clone(), dep: self.dep.clone() }
    }
}

impl<Op: SegtreeOp> Debug for Segtree<Op> where Op::Value: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut seg = self.clone();
        seg.push_all();
        write!(f, "{:?}", &seg.tree[self.len()..])
    }
}



impl<Op: SegtreeOp, I: SliceIndex<[Op::Value]>> Index<I> for Segtree<Op> {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&self.tree[self.len()..], index)
    }
}
