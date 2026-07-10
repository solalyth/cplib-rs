use std::{cell::Cell, ops::{Deref, DerefMut}, ptr::NonNull};

pub trait SplayOp: Sized {
    type D;
    type Value;
    
    fn new_dat(arg: Self::Value) -> Self::D;
    // fn push(node: &mut Node<Self>);
    fn recalc(node: Ref<Self>);
}



pub struct Node<Op: SplayOp> {
    parent: Option<Ref<Op>>,
    child: [Option<Ref<Op>>; 2],
    dat: Op::D,
}

pub struct Ref<Op: SplayOp>(NonNull<Node<Op>>);

fn connect<Op: SplayOp>(p: Option<Ref<Op>>, c: Option<Ref<Op>>, pos: bool) {
    if let Some(mut p) = p { p.child[pos as usize] = c; }
    if let Some(mut c) = c { c.parent = p; }
}

impl<Op: SplayOp> Ref<Op> {
    fn new(arg: Op::Value) -> Self {
        let node = Node {
            parent: None,
            child: [None, None],
            dat: Op::new_dat(arg),
        };
        Ref(NonNull::new(Box::into_raw(Box::new(node))).unwrap())
    }
    
    /// `parent` の位置に `self` が来るよう回転する。
    fn rotate(self, p: Self, pos: bool) {
        connect(p.parent, Some(self), p.parent.map_or(false, |g| g.child[1] == Some(p)));
        connect(Some(p), self.child[!pos as usize], pos);
        connect(Some(self), Some(p), !pos);
    }
    
    fn splay(self) {
        while let Some(p) = self.parent {
            let pos = p.child[1] == Some(self);
            let Some(pp) = p.parent else { self.rotate(p, pos); Op::recalc(p); Op::recalc(self); return; };
            
            if pos == (pp.child[1] == Some(p)) {
                p.rotate(pp, pos); self.rotate(p, pos); // zig-zig
            } else {
                self.rotate(p, pos); self.rotate(pp, !pos); // zig-zag
            }
            
            Op::recalc(pp);
            Op::recalc(p);
            Op::recalc(self);
        }
    }
}


impl<Op: SplayOp> Clone for Ref<Op> { fn clone(&self) -> Self { Ref(self.0) } }
impl<Op: SplayOp> Copy for Ref<Op> {}
impl<Op: SplayOp> PartialEq for Ref<Op> { fn eq(&self, other: &Self) -> bool { self.0 == other.0 } }
impl<Op: SplayOp> Deref for Ref<Op> { type Target = Node<Op>; fn deref(&self) -> &Node<Op> { unsafe { self.0.as_ref() } } }
impl<Op: SplayOp> DerefMut for Ref<Op> { fn deref_mut(&mut self) -> &mut Node<Op> { unsafe { self.0.as_mut() } } }





pub struct SplayTree<Op: SplayOp>(Cell<Option<Ref<Op>>>);

impl<Op: SplayOp> SplayTree<Op> {
    pub fn new() -> Self { Self(Cell::new(None)) }
    
    pub fn insert(&self, idx: usize, value: Op::Value) {
        
    }
}
