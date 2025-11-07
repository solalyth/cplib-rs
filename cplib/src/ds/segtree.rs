use std::{fmt::Debug, mem::replace, ops::{Index, IndexMut, RangeBounds}, slice::SliceIndex};
use crate::cplib::util::func::to_bounds;

/// Operator for [`Segtree`], [`crate::ds::sparse_segtree::SparseSegtree`]
/// 
/// `apply` を利用しないなら `id_value`, `prod_value` のみの実装で動く。
#[allow(unused_variables)]
pub trait SegtreeOp: Sized {
    const BEATS: bool = false;
    
    type Value: Clone + Debug;
    type Lazy: Clone;
    
    /// `Value` の単位元を返す。
    fn id_value() -> Self::Value;
    
    /// `Value` の積を返す。
    fn prod_value(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    
    /// `value` に `lazy` を作用させる。成功した場合は作用させて `true` を返し、失敗した場合は何もせず `false` を返すこと。
    fn act_value(value: &mut Self::Value, lazy: &Self::Lazy) -> bool { true }
    
    /// `lazy` の上から `ad` を合成させる。
    fn prod_lazy(lazy: &mut Self::Lazy, ad: &Self::Lazy) {}
    
    fn segtree_new(len: usize) -> Segtree<Self> { Segtree::new(len) }
    fn segtree_from_iter(iter: impl Iterator<Item = Self::Value>) -> Segtree<Self> { Segtree::from_iter(iter) }
}




/// 遅延可能 Beats! 可能セグメント木
/// 
/// `lazy[i]` には `tree[i]` の子孫が受ける作用が入る。特に、`tree[i]` は `lazy[i]` が既に作用されている。
/// 
/// # 搭載機能
/// 
/// - `Clone`, `Debug`
pub struct Segtree<Op: SegtreeOp> {
    tree: Vec<Op::Value>,
    /// `lazy[i]` = `i` の子孫が反映待ちである `Lazy`
    lazy: Vec<Option<Op::Lazy>>,
    depth: u32
}



impl<Op: SegtreeOp> Segtree<Op> {
    pub fn new(len: usize) -> Self {
        let depth = (len.max(2)-1).ilog2() + 2;
        Segtree { tree: vec![Op::id_value(); 1<<depth], lazy: vec![None; 1<<depth], depth }
    }
    
    /// 最下層の長さを返す。これは [`Segtree::new`] で指定した長さと異なる可能性がある。
    pub fn len(&self) -> usize { 1 << self.depth-1 }
    
    pub fn get(&mut self, mut i: usize) -> &Op::Value {
        i += self.len();
        for j in (1..self.depth).rev() { self.push(i >> j); }
        &self.tree[i]
    }
    
    pub fn set(&mut self, mut i: usize, f: impl FnOnce(&mut Op::Value)) {
        i += self.len();
        for j in (1..self.depth).rev() { self.push(i >> j); }
        f(&mut self.tree[i]);
        for j in 1..self.depth { self.update(i >> j); }
    }
    
    pub fn entry(&mut self) -> Entry<'_, Op> {
        for i in 1..self.len() { self.push(i); }
        Entry { seg: self, changed: false }
    }
    
    pub fn fold(&mut self, range: impl RangeBounds<usize>) -> Op::Value {
        let [mut l, mut r] = to_bounds(range, self.len()).map(|v| v+self.len());
        if r == self.len() { return Op::id_value(); }
        let (mut rl, mut rr) = (Op::id_value(), Op::id_value());
        
        for i in (1..self.depth).rev() { self.push(l >> i); self.push(r-1 >> i); }
        
        while l < r {
            if l&1 == 1 { rl = Op::prod_value(&rl, &self.tree[l]); l += 1; }
            if r&1 == 1 { rr = Op::prod_value(&self.tree[r-1], &rr); }
            l >>= 1; r >>= 1;
        }
        
        Op::prod_value(&rl, &rr)
    }
    
    pub fn apply(&mut self, range: impl RangeBounds<usize>, lazy: Op::Lazy) {
        let [l, r] = to_bounds(range, self.len()).map(|v| v + self.len());
        if r == self.len() { return; }
        
        for i in (1..self.depth).rev() { self.push(l >> i); self.push(r-1 >> i); }
        
        let (mut s, mut t) = (l, r);
        while s < t {
            if s&1 == 1 { self.node_apply(s, &lazy); s += 1; }
            if t&1 == 1 { t -= 1; self.node_apply(t, &lazy); }
            s >>= 1; t >>= 1;
        }
        
        for i in 1..self.depth {
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
        
        for i in (1..self.depth).rev() { self.push(r >> i); }
        
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
        
        for i in (1..self.depth).rev() { self.push(l-1 >> i); }
        
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
        if Op::BEATS {
            self.comp_lazy(i, lazy);
            if !Op::act_value(&mut self.tree[i], lazy) {
                self.push(i);
                self.update(i);
            }
        } else {
            Op::act_value(&mut self.tree[i], lazy);
            self.comp_lazy(i, lazy);
        }
    }
    
    /// `i` の子に `lazy[i]` を作用・伝搬させる。
    #[track_caller]
    fn push(&mut self, i: usize) {
        debug_assert!(i < self.len());
        
        let Some(lazy) = replace(&mut self.lazy[i], None) else { return };
        self.node_apply(2*i, &lazy);
        self.node_apply(2*i+1, &lazy);
    }
    
    /// `tree[i]` を子から再計算する。
    fn update(&mut self, i: usize) {
        debug_assert!(i < self.len());
        debug_assert!(self.lazy[i].is_none());
        
        self.tree[i] = Op::prod_value(&self.tree[2*i], &self.tree[2*i+1]);
    }
    
    /// `lazy[i]` の上に `ad` を合成する。
    fn comp_lazy(&mut self, i: usize, ad: &Op::Lazy) {
        if let Some(lazy) = &mut self.lazy[i] { Op::prod_lazy(lazy, ad); } else { self.lazy[i] = Some(ad.clone()); }
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
        Self { tree: self.tree.clone(), lazy: self.lazy.clone(), depth: self.depth.clone() }
    }
}

impl<Op: SegtreeOp> Debug for Segtree<Op> where Op::Value: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut seg = self.clone();
        for i in 1..seg.len() { seg.push(i); }
        write!(f, "{:?}", &seg.tree[self.len()..])
    }
}

impl<Op: SegtreeOp> FromIterator<Op::Value> for Segtree<Op> {
    fn from_iter<T: IntoIterator<Item = Op::Value>>(iter: T) -> Self {
        let v = iter.into_iter().collect::<Vec<_>>();
        let mut seg = Self::new(v.len());
        let len = seg.len();
        for (i, v) in v.into_iter().enumerate() { seg.tree[len+i] = v; }
        for i in (1..seg.len()).rev() { seg.update(i); }
        seg
    }
}



pub struct Entry<'a, Op: SegtreeOp> {
    seg: &'a mut Segtree<Op>,
    changed: bool
}

impl<Op: SegtreeOp, I: SliceIndex<[Op::Value]>> Index<I> for Entry<'_, Op> {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&self.seg.tree[self.seg.len()..], index)
    }
}

impl<Op: SegtreeOp, I: SliceIndex<[Op::Value]>> IndexMut<I> for Entry<'_, Op> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let len = self.seg.len();
        self.changed = true;
        IndexMut::index_mut(&mut self.seg.tree[len..], index)
    }
}

impl<Op: SegtreeOp> Drop for Entry<'_, Op> {
    fn drop(&mut self) {
        if self.changed {
            for i in (1..self.seg.len()).rev() { self.seg.update(i); }
        }
    }
}
