use std::{cell::Cell, cmp::Ordering, fmt::Debug, ops::{Bound, Deref, DerefMut, RangeBounds}, ptr::NonNull};
use crate::cplib::util::func::to_bounds;

// use crate::util::func::to_bounds;

pub trait OrderedOp {
    type Key: Debug;
    type Value: Debug;
    type Acc: Clone;
    
    fn cmp_key(l: &Self::Key, r: &Self::Key) -> Ordering;
    fn to_acc(v: &Self::Value) -> Self::Acc;
    fn prod_acc(lhs: &Self::Acc, rhs: &Self::Acc) -> Self::Acc;
}

/// [`OrderedOp::Key`] によって順序付けられた列。
/// 
/// 内部は splay 木である。
pub struct Ordered<Op: OrderedOp>(Cell<Option<NodeRef<Op>>>);

impl<Op: OrderedOp> Ordered<Op> {
    pub fn new() -> Self { Self(None.into()) }
    pub fn is_empty(&self) -> bool { self.0.get().is_none() }
    pub fn len(&self) -> usize { self.0.get().map_or(0, |r| r.cnt()) }
    
    fn splay(&self, ptr: NodeRef<Op>) {
        ptr.splay();
        self.0.set(Some(ptr));
    }
    
    /// # Panics
    /// 
    /// if `Ordered` is empty.
    pub fn root(&self) -> &Op::Value {
        unsafe { &self.0.get().unwrap().0.as_ref().value }
    }
    
    /// # Panics
    /// 
    /// if `Ordered` is empty.
    pub fn root_mut<T>(&mut self, f: impl FnOnce(&mut Op::Value) -> T) -> T {
        let mut r = self.0.get().unwrap();
        let res = f(&mut r.value);
        r.update();
        res
    }
    
    /// `idx` 番目のノードを root にする。
    /// 
    /// # Panics
    /// 
    /// if not `idx < self.len`
    pub fn nth(&self, mut idx: usize) {
        assert!(idx < self.len());
        let mut ptr = self.0.get().unwrap();
        loop {
            match idx.cmp(&ptr.cnt.0) {
                Ordering::Less => { ptr = ptr.child[0].unwrap(); }
                Ordering::Equal => { self.splay(ptr); return; }
                Ordering::Greater => { idx -= ptr.cnt.0+1; ptr = ptr.child[1].unwrap(); }
            }
        }
    }
    
    /// `pred(idx-1) == true && pred(idx) = false` となる `idx` を返す。
    pub fn partition_point(&self, pred: impl Fn(&Op::Key) -> bool) -> usize {
        let Some(mut ptr) = self.0.get() else { return 0; };
        let mut tmp = None;
        loop {
            let pos = pred(&ptr.key);
            if !pos { tmp = Some(ptr); }
            if let Some(c) = ptr.child[pos as usize] { ptr = c; } else { break; }
        }
        let ptr = tmp.unwrap_or(ptr);
        self.splay(ptr);
        ptr.cnt.0 - tmp.is_none() as usize
    }
    
    fn split_root(self) -> [Self; 2] {
        let Some(mut r) = self.0.get() else { return [Self::new(), self]; };
        let Some(mut l) = std::mem::replace(&mut r.child[0], None) else { return [Self::new(), self]; };
        l.parent = None;
        r.update();
        [Ordered(Some(l).into()), self]
    }
    
    /// `[self[..idx], self[idx..]]` に分割する。はみ出してもよい。
    pub fn split_at(self, idx: usize) -> [Self; 2] {
        if idx == 0 { return [Self::new(), self]; }
        if self.len() <= idx { return [self, Self::new()]; }
        self.nth(idx);
        self.split_root()
    }
    
    pub fn split3_key(self, range: impl RangeBounds<Op::Key>) -> [Self; 3] {
        let mid = Ordered(self.0.clone());
        let idx = match range.end_bound() {
            Bound::Included(x) => mid.upper_bound(x),
            Bound::Excluded(x) => mid.lower_bound(x),
            Bound::Unbounded => { mid.last(); mid.len() }
        };
        let [mid, r] = if idx != mid.len() { mid.split_root() } else { [mid, Self::new()] };
        match range.start_bound() {
            Bound::Included(x) => { mid.lower_bound(x); },
            Bound::Excluded(x) => { mid.upper_bound(x); },
            Bound::Unbounded => { mid.first(); }
        };
        let [l, mid] = mid.split_root();
        [l, mid, r]
    }
    
    pub fn merge(l: Self, r: Self) -> Self {
        r.first();
        let Some(rr) = r.0.get() else { return l; };
        rr.set_child(l.0.get(), false);
        rr.update();
        r
    }
    
    
    
    /// lower bound を求める。
    /// 
    /// `A[root-1] < key <= A[root]` となる `root` が存在すれば根にする。存在しない場合は、一番右のノードを根にする。
    pub fn lower_bound(&self, key: &Op::Key) -> usize {
        self.partition_point(|k| Op::cmp_key(k, key).is_lt())
    }
    
    /// upper bound を求める。
    /// 
    /// `A[root-1] <= key < A[root]` となる `root` が存在すれば根にする。存在しない場合は、一番右のノードを根にする。
    pub fn upper_bound(&self, key: &Op::Key) -> usize {
        self.partition_point(|k| Op::cmp_key(k, key).is_le())
    }
    
    /// `key` が等しい区間の一番左に挿入する。
    pub fn insert(&mut self, key: Op::Key, value: Op::Value) {
        let idx = self.lower_bound(&key);
        let node = NodeRef::new(key, value);
        let Some(mut p) = self.0.get() else { self.0.set(Some(node)); return; };
        if idx == self.len() { node.set_child(Some(p), false); node.update(); self.0.set(Some(node)); return; }
        node.set_child(p.child[0], false);
        node.set_child(Some(p), true);
        p.child[0] = None;
        p.update(); node.update();
        self.0.set(Some(node));
    }
    
    pub fn insert_replace(&mut self, key: Op::Key, init: Op::Value, update: impl FnOnce(&mut Op::Value)) {
        let idx = self.lower_bound(&key);
        let Some(mut p) = self.0.get() else { self.0.set(Some(NodeRef::new(key, init))); return; };
        if Op::cmp_key(&key, &p.key).is_eq() { update(&mut p.value); p.update(); return; }

        let node = NodeRef::new(key, init);
        if idx == self.len() { node.set_child(Some(p), false); node.update(); self.0.set(Some(node)); return; }
        node.set_child(p.child[0], false);
        node.set_child(Some(p), true);
        p.child[0] = None;
        p.update(); node.update();
        self.0.set(Some(node));
    }
    
    pub fn fold(&self, range: impl RangeBounds<usize>) -> Option<Op::Acc> {
        let [l, r] = to_bounds(range, self.len());
        if r == 0 { return None; }
        let [mid, r] = Ordered(self.0.clone()).split_at(r);
        let [l, mid] = mid.split_at(l);
        let res = mid.0.get().unwrap().acc.clone();
        self.0.set(Ordered::merge(Ordered::merge(l, mid), r).0.get());
        Some(res)
    }
    
    pub fn fold_key(&self, range: impl RangeBounds<Op::Key>) -> Option<Op::Acc> {
        let [l, mid, r] = Ordered(self.0.clone()).split3_key(range);
        let res = mid.0.get().unwrap().acc.clone();
        self.0.set(Ordered::merge(Ordered::merge(l, mid), r).0.get());
        Some(res)
    }
    
    
    
    pub fn next(&mut self) -> bool {
        let Some(p) = self.0.get() else { return false; };
        let Some(ptr) = p.child[1] else { return false; };
        self.splay(ptr.first());
        true
    }
    
    pub fn back(&mut self) -> bool {
        let Some(p) = self.0.get() else { return false; };
        let Some(ptr) = p.child[0] else { return false; };
        self.splay(ptr.last());
        true
    }
    
    pub fn first(&self) {
        let Some(mut ptr) = self.0.get() else { return; };
        ptr = ptr.first();
        self.splay(ptr);
    }

    pub fn last(&self) {
        let Some(mut ptr) = self.0.get() else { return; };
        ptr = ptr.last();
        self.splay(ptr);
    }
}

impl<Op: OrderedOp> Debug for Ordered<Op> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", debug(self.0.get(), f.alternate()))
    }
}





pub struct Node<Op: OrderedOp> {
    parent: Option<NodeRef<Op>>,
    child: [Option<NodeRef<Op>>; 2],
    pub key: Op::Key,
    pub value: Op::Value,
    pub acc: Op::Acc,
    /// (左子のサイズ, 右子のサイズ)
    pub cnt: (usize, usize)
}

pub struct NodeRef<Op: OrderedOp>(NonNull<Node<Op>>);

impl<Op: OrderedOp> NodeRef<Op> {
    fn new(key: Op::Key, value: Op::Value) -> Self {
        unsafe {
            NodeRef(NonNull::new_unchecked(Box::leak(Box::new(Node { parent: None, child: [None, None], key, acc: Op::to_acc(&value), value, cnt: (0, 0) }))))
        }
    }
    
    fn cnt(self) -> usize { self.cnt.0 + self.cnt.1 + 1 }
    
    fn set_child(mut self, c: Option<Self>, pos: bool) {
        self.child[pos as usize] = c;
        if let Some(mut c) = c { c.parent = Some(self); }
    }
    
    /// `p` の位置に `self` が来るよう rotate する。
    /// 
    /// # Panics
    /// 
    /// if `self.parent == None`
    fn rotate(mut self, p: Self, pos: bool) {
        // todo
        self.parent = p.parent;
        if let Some(pp) = p.parent { pp.set_child(Some(self), pp.child[1] == Some(p)); }
        p.set_child(self.child[!pos as usize], pos);
        self.set_child(Some(p), !pos);
        p.update(); self.update();
    }
    
    /// `self` が根になるように適切に回転する。
    fn splay(self) {
        while let Some(p) = self.parent {
            let pos = p.child[1] == Some(self);
            let Some(pp) = p.parent else { self.rotate(p, pos); return; };
            if pos == (pp.child[1] == Some(p)) {
                // zig-zig
                p.rotate(pp, pos); self.rotate(p, pos);
            } else {
                // zig-zag
                self.rotate(p, pos); self.rotate(pp, !pos);
            }
        }
    }
    
    /// `cnt`, `acc` を再計算する。
    fn update(mut self) {
        self.cnt = (0, 0);
        self.acc = Op::to_acc(&self.value);
        if let Some(c) = self.child[0] {
            self.cnt.0 = c.cnt();
            self.acc = Op::prod_acc(&c.acc, &self.acc);
        }
        if let Some(c) = self.child[1] {
            self.cnt.1 = c.cnt();
            self.acc = Op::prod_acc(&self.acc, &c.acc);
        }
    }
    
    
    fn first(mut self) -> NodeRef<Op> {
        while let Some(c) = self.child[0] { self = c; }
        self
    }
    
    fn last(mut self) -> NodeRef<Op> {
        while let Some(c) = self.child[1] { self = c; }
        self
    }
}


impl<Op: OrderedOp> Deref for NodeRef<Op> {
    type Target = Node<Op>; fn deref(&self) -> &Self::Target { unsafe { self.0.as_ref() } }
}

impl<Op: OrderedOp> DerefMut for NodeRef<Op> {
    fn deref_mut(&mut self) -> &mut Self::Target { unsafe { self.0.as_mut() } }
}

impl<Op: OrderedOp> Clone for NodeRef<Op> { fn clone(&self) -> Self { Self(self.0.clone()) } }

impl<Op: OrderedOp> Copy for NodeRef<Op> {}

impl<Op: OrderedOp> PartialEq for NodeRef<Op> {
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}
impl<Op: OrderedOp> Eq for NodeRef<Op> {}



fn debug<Op: OrderedOp>(ptr: Option<NodeRef<Op>>, alt: bool) -> String where Op::Key: Debug, Op::Value: Debug {
    let mut li = String::new();
    let Some(p) = ptr else { return String::from("[]"); };
    let mut stk = vec![(p, true, 0)];
    while let Some((p, is_in, indent)) = stk.pop() {
        if is_in {
            if let Some(r) = p.child[1] { stk.push((r, true, indent+1)); }
            stk.push((p, false, indent));
            if let Some(l) = p.child[0] { stk.push((l, true, indent+1)); }
        } else {
            if alt {
                li += "\n";
                for _ in 0..indent { li += "    "; }
                li += &format!("key={:?}, value={:?}", p.key, p.value);
            } else {
                li += &format!("{:?}: {:?}, ", p.key, p.value);
            }
        }
    }
    if alt {
        li.remove(0); li
    } else {
        format!("{{{}}}", &li[..li.len()-2])
    }
}
