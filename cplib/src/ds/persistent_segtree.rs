use std::ops::RangeBounds;
use crate::cplib::util::func::to_bounds;
use std::fmt::Debug;

/// Operator for [`PersistentSegtree`]
#[allow(unused_variables)]
pub trait PersistentSegtreeOp: Sized {
    const BEATS: bool = false;
    
    type Value: Clone + Debug;
    type Lazy: Clone;
    
    /// `Value` の単位元を返す。
    fn id_value() -> Self::Value;
    
    /// `Value` の積を返す。
    fn prod_value(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    
    /// `value` に `lazy` を作用させる。
    fn act_value(value: &mut Self::Value, lazy: &Self::Lazy) {}
    
    /// `lazy` の上から `ad` を合成させる。
    fn prod_lazy(lazy: &mut Self::Lazy, ad: &Self::Lazy) {}
    
    fn segtree_new(len: usize) -> PersistentSegtree<Self> { PersistentSegtree::new(len) }
}



type Node<Op> = (<Op as PersistentSegtreeOp>::Value, Option<<Op as PersistentSegtreeOp>::Lazy>, [usize; 2]);

pub struct PersistentSegtree<Op: PersistentSegtreeOp> {
    pub pool: Vec<Node<Op>>,
    depth: usize
}

impl<Op: PersistentSegtreeOp> PersistentSegtree<Op> {
    pub fn new(len: usize) -> Self {
        let depth = (len.max(2)-1).ilog2() as usize + 2;
        PersistentSegtree { pool: vec![], depth }
    }
    
    pub fn len(&self) -> usize { 1 << self.depth-1 }
    
    /// `lazy[idx]` を子に伝搬させる。
    pub fn push(&mut self, idx: usize) {
        if let Some(lazy) = &self.pool[idx].1 {
            let lazy = lazy.clone();
            for i in 0..2 {
                let mut node = self.pool[self.pool[idx].2[i]].clone();
                Op::act_value(&mut node.0, &lazy);
                if let Some(l) = &mut node.1 { Op::prod_lazy(l, &lazy); } else { node.1 = Some(lazy.clone()); }
                self.pool[idx].2[i] = self.pool.len();
                self.pool.push(node);
            }
        }
    }
    
    pub fn update(&mut self, idx: usize) {
        let [l, r] = self.pool[idx].2;
        self.pool[idx].0 = Op::prod_value(&self.pool[l].0, &self.pool[r].0);
    }
    
    pub fn extend(&mut self, iter: impl ExactSizeIterator<Item = Op::Value>) -> usize {
        assert!(iter.len() <= self.len());
        let rt = self.pool.len();
        for _ in 0..1<<self.depth { self.pool.push((Op::id_value(), None, [!0, !0])); }
        let p = &mut self.pool[rt..];
        for (i, v) in iter.enumerate() { p[(1<<self.depth-1)+i].0 = v; }
        for i in (1..1<<self.depth-1).rev() {
            p[i].0 = Op::prod_value(&p[2*i].0, &p[2*i+1].0);
            p[i].2 = [rt+2*i, rt+2*i+1];
        }
        rt+1
    }
    
    pub fn get(&mut self, mut root: usize, idx: usize) -> &Op::Value {
        assert!(idx < self.len());
        for d in (0..self.depth-1).rev() {
            self.push(root);
            root = self.pool[root].2[idx>>d & 1];
        }
        &self.pool[root].0
    }
    
    pub fn set(&mut self, mut root: usize, idx: usize, f: impl FnOnce(&mut Op::Value)) -> usize {
        assert!(idx < self.len());
        let rt = self.pool.len();
        for d in (0..self.depth-1).rev() {
            self.push(root);
            let mut t = self.pool[root].clone();
            t.2[idx>>d & 1] = self.pool.len()+1;
            self.pool.push(t);
            root = self.pool[root].2[idx>>d & 1];
        }
        let mut t = self.pool[root].clone();
        f(&mut t.0);
        self.pool.push(t);
        for i in (0..self.depth-1).rev() { self.update(rt+i); }
        rt
    }
    
    fn _fold(&mut self, root: usize, l: usize, r: usize, d: usize) -> Op::Value {
        if (l, r) == (0, 1<<d) { return self.pool[root].0.clone(); }
        self.push(root);
        let [cl, cr] = self.pool[root].2;
        let m = 1<<(d-1);
        
        if r <= m {
            self._fold(cl, l, r, d-1)
        } else if m <= l {
            self._fold(cr, l-m, r-m, d-1)
        } else {
            Op::prod_value(&self._fold(cl, l, m, d-1), &self._fold(cr, 0, r-m, d-1))
        }
    }
    
    pub fn fold(&mut self, root: usize, range: impl RangeBounds<usize>) -> Op::Value {
        let [l, r] = to_bounds(range, self.len());
        if r == 0 { return Op::id_value(); }
        self._fold(root, l, r, self.depth-1)
    }
    
    // pub fn _apply(&mut self, p: usize, l: usize, r: usize, d: usize, lazy: &Op::Lazy) {
        
    // }
    
    // pub fn apply(&mut self, root: usize, range: impl RangeBounds<usize>, lazy: Op::Lazy) -> usize {
        
    // }
}
