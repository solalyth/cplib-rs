use std::{cell::Cell, cmp::Ordering, fmt::{Debug, Display}, ops::{Deref, DerefMut}, ptr::NonNull};

// use crate::util::func::to_bounds;

pub trait OrderedOp {
    type Key;
    type Value;
    type Acc;
    
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
    pub fn len(&self) -> usize { self.0.get().map_or(0, |p| p.cnt()) }
    /// index of `root` と等しい。
    pub fn len_left(&self) -> usize { self.0.get().map_or(0, |p| p.cnt.0) }
    pub fn len_right(&self) -> usize { self.0.get().map_or(0, |p| p.cnt.1) }
    
    /// # Panics
    /// 
    /// if `Ordered` is empty.
    pub fn get_root(&self) -> &Node<Op> {
        unsafe { self.0.get().unwrap().0.as_ref() }
    }
    
    /// # Panics
    /// 
    /// if `Ordered` is empty.
    pub fn set_root(&self, f: impl FnOnce(&mut Op::Value)) {
        let mut ptr = self.0.get().unwrap();
        f(&mut ptr.value);
        ptr.update();
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
                Ordering::Equal => { ptr.splay(); return; }
                Ordering::Greater => { idx -= ptr.cnt.0+1; ptr = ptr.child[1].unwrap(); }
            }
        }
    }
    
    pub fn partition_point(&self, pred: impl Fn(&Op::Key) -> bool) -> usize {
        let Some(mut ptr) = self.0.get() else { return 0; };
        let mut tmp = None;
        loop {
            let pos = pred(&ptr.key);
            if !pos { tmp = Some(ptr); }
            if let Some(c) = ptr.child[pos as usize] { ptr = c; } else { break; }
        }
        let root = tmp.unwrap_or(ptr);
        root.splay();
        self.0.set(Some(root));
        root.cnt.0 + root.child[1].is_none() as usize
    }
    
    /// lower bound を求める。`#{ elem | elem.key < key }` に等しい。
    /// 
    /// `A[root-1] < key <= A[root]` となる `root` が存在すれば根にする。存在しない場合は、一番右のノードを根にする。
    pub fn lower_bound(&self, key: &Op::Key) -> usize {
        self.partition_point(|k| Op::cmp_key(k, key).is_lt())
    }
    
    /// upper bound を求める。`#{ elem | elem.key <= key }` に等しい。
    /// 
    /// `A[root-1] <= key < A[root]` となる `root` が存在すれば根にする。存在しない場合は、一番右のノードを根にする。
    pub fn upper_bound(&self, key: &Op::Key) -> usize {
        self.partition_point(|k| Op::cmp_key(k, key).is_le())
    }
    
    pub fn insert(&mut self, key: Op::Key, value: Op::Value, replace: bool) {
        let idx = self.lower_bound(&key);
        let Some(mut p) = self.0.get() else { self.0.set(Some(NodeRef::new(key, value))); return; };
        if replace && Op::cmp_key(&key, &p.key).is_eq() { p.value = value; p.update(); return; }
        let node = NodeRef::new(key, value);
        // node が最右に来るべきとき
        if idx == self.len() { node.set_child(Some(p), false); node.update(); self.0.set(Some(node)); return; }
        node.set_child(p.child[0], false);
        node.set_child(Some(p), true);
        p.child[0] = None;
        p.update(); node.update();
        self.0.set(Some(node));
    }
    
    pub fn next(&mut self) -> bool {
        let Some(p) = self.0.get() else { return false; };
        let Some(mut ptr) = p.child[1] else { return false; };
        while let Some(c) = ptr.child[0].or(ptr.child[1]) { ptr = c; }
        ptr.splay();
        self.0.set(Some(ptr));
        true
    }
    
    pub fn next_back(&mut self) -> bool {
        let Some(p) = self.0.get() else { return false; };
        let Some(mut ptr) = p.child[0] else { return false; };
        while let Some(c) = ptr.child[1].or(ptr.child[0]) { ptr = c; }
        ptr.splay();
        self.0.set(Some(ptr));
        true
    }
    
    pub fn from_iter(replace: bool, iter: impl Iterator<Item = (Op::Key, Op::Value)>) -> Self {
        let mut res = Ordered::new();
        for (k, v) in iter { res.insert(k, v, replace); }
        res
    }
}

impl<Op: OrderedOp> Debug for Ordered<Op> where Op::Key: Display, Op::Value: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Some(p) = self.0.get() else { return write!(f, "Empty"); };
        let mut li = String::new();
        let mut stk = vec![(p, true)];
        while let Some((p, is_in)) = stk.pop() {
            if is_in {
                if let Some(r) = p.child[1] { stk.push((r, true)); }
                stk.push((p, false));
                if let Some(l) = p.child[0] { stk.push((l, true)); }
            } else {
                li += &format!(", {}({})", p.value, p.key);
            }
        }
        if li.is_empty() {
            write!(f, "empty")
        } else {
            write!(f, "[{}]", &li[2..])
        }
    }
}





pub struct Node<Op: OrderedOp> {
    parent: Option<NodeRef<Op>>,
    child: [Option<NodeRef<Op>>; 2],
    pub key: Op::Key,
    pub value: Op::Value,
    pub acc: Op::Acc,
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
    
    // #[allow(dead_code)]
    // fn disjoint_left(mut root: Self) -> Option<Self> {
    //     let Some(mut l) = replace(&mut root.child[0], None) else { return None; };
    //     l.parent = None;
    //     root.update();
    //     l.update();
    //     Some(l)
    // }
    
    // #[allow(dead_code)]
    // fn disjoint_right(mut root: Self) -> Option<Self> {
    //     let Some(mut r) = replace(&mut root.child[1], None) else { return None; };
    //     r.parent = None;
    //     root.update();
    //     r.update();
    //     Some(r)
    // }
    
    
    
    
    
    
    /// `p` の位置に `self` が来るよう rotate する。
    /// 
    /// # Panics
    /// 
    /// if `self.parent == None`
    fn rotate(mut self, p: Self, pos: bool) {
        self.parent = p.parent;
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
