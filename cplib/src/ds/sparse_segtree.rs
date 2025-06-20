use std::mem::swap;
use std::ptr::NonNull;
use std::ops::{Deref, DerefMut, RangeBounds};
use crate::cplib::util::func::to_bounds;



pub trait SparseSegtreeOp {
    type Value: Clone;
    fn id_value() -> Self::Value;
    fn prod_value(l: &Self::Value, r: &Self::Value) -> Self::Value;
}



/// 疎セグメント木
/// 
/// 遅延は出来ない。
pub struct SparseSegtree<Op: SparseSegtreeOp> {
    len: usize,
    ptr: Option<NodeRef<Op>>
}

struct NodeRef<Op: SparseSegtreeOp>(NonNull<Node<Op>>);

struct Node<Op: SparseSegtreeOp> {
    idx: usize,
    value: Op::Value,
    prod: Op::Value,
    left: Option<NodeRef<Op>>,
    right: Option<NodeRef<Op>>
}




impl<Op: SparseSegtreeOp> SparseSegtree<Op> {
    pub fn new(len: usize) -> Self {
        Self { len, ptr: None }
    }
    
    pub fn set(&mut self, idx: usize, value: Op::Value) {
        NodeRef::set(&mut self.ptr, 0, self.len, idx, value);
    }
    
    pub fn get(&self, idx: usize) -> Op::Value {
        NodeRef::get(&self.ptr, 0, self.len, idx)
    }
    
    pub fn fold(&self, range: impl RangeBounds<usize>) -> Op::Value {
        let [l, r] = to_bounds(range, self.len);
        NodeRef::fold(&self.ptr, 0, self.len, l, r)
    }
}



impl<Op: SparseSegtreeOp> NodeRef<Op> {
    fn new(idx: usize, value: Op::Value) -> NodeRef<Op> {
        unsafe {
            NodeRef(NonNull::new_unchecked(Box::leak(Box::new(Node { idx, prod: value.clone(), value, left: None, right: None }))))
        }
    }
    
    /// `ptr`: `l..r` の範囲を担当する Node
    fn set(ptr: &mut Option<NodeRef<Op>>, l: usize, r: usize, mut idx: usize, mut value: Op::Value) {
        let Some(ptr) = ptr else { *ptr = Self::new(idx, value).into(); return; };
        if ptr.idx == idx { ptr.value = value; ptr.update(); return; }
        let c = (l+r)/2;
        if (idx < c) != (idx < ptr.idx) { swap(&mut ptr.idx, &mut idx); swap(&mut ptr.value, &mut value); }
        if idx < c {
            // l <= idx < c, node.ptr
            NodeRef::set(&mut ptr.left, l, c, idx, value);
        } else {
            // node.ptr, c <= idx < r
            NodeRef::set(&mut ptr.right, c, r, idx, value);
        }
        ptr.update();
    }
    
    fn update(&mut self) {
        let mut prod = self.value.clone();
        if let Some(left) = self.left.as_ref() {
            prod = Op::prod_value(&left.prod, &prod);
        }
        if let Some(right) = self.right.as_ref() {
            prod = Op::prod_value(&prod, &right.prod);
        }
        self.prod = prod;
    }
    
    fn get(ptr: &Option<NodeRef<Op>>, l: usize, r: usize, idx: usize) -> Op::Value {
        let Some(ptr) = ptr else { return Op::id_value(); };
        if ptr.idx == idx { return ptr.value.clone(); }
        let c = (l+r)/2;
        if idx < c { NodeRef::get(&ptr.left, l, c, idx) } else { NodeRef::get(&ptr.right, c, r, idx) }
    }
    
    fn fold(ptr: &Option<NodeRef<Op>>, l: usize, r: usize, fl: usize, fr: usize) -> Op::Value {
        let Some(ptr) = ptr else { return Op::id_value(); };
        if r <= fl || fr <= l { return Op::id_value(); }
        if fl <= l && r <= fr { return ptr.prod.clone(); }
        let c = (l+r)/2;
        let cent = if (fl..fr).contains(&ptr.idx) { ptr.value.clone() } else { Op::id_value() };
        let l = NodeRef::fold(&ptr.left, l, c, fl, fr);
        let r = NodeRef::fold(&ptr.right, c, r, fl, fr);
        Op::prod_value(&l, &Op::prod_value(&cent, &r))
    }
}

impl<Op: SparseSegtreeOp> Deref for NodeRef<Op> {
    type Target = Node<Op>;
    
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl<Op: SparseSegtreeOp> DerefMut for NodeRef<Op> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}
