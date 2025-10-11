use std::{collections::VecDeque, fmt::Debug};



pub trait FoldableDequeOp {
    type Value;
    type Acc: Clone;
    fn to_acc(v: &Self::Value) -> Self::Acc;
    fn prod_acc(l: &Self::Acc, r: &Self::Acc) -> Self::Acc;
    fn id_acc() -> Self::Acc;
}



/// - `push, pop`: amortized `O(1)`
/// - `fold`: `O(1)`
/// 
/// # Implemented Traits
/// 
/// - `Clone`, `Debug`, `FromIterator<T>`
pub struct FoldableDeque<Op: FoldableDequeOp> {
    deque: VecDeque<Op::Value>,
    front: Vec<Op::Acc>,
    back: Vec<Op::Acc>,
    e: Op::Acc
}

impl<Op: FoldableDequeOp> FoldableDeque<Op> {
    pub fn new() -> Self {
        Self { deque: VecDeque::new(), front: vec![], back: vec![], e: Op::id_acc() }
    }
    
    pub fn deque(&self) -> &VecDeque<Op::Value> { &self.deque }
    pub fn len(&self) -> usize { self.deque.len() }
    
    fn fold_front(&self) -> &Op::Acc { self.front.last().unwrap_or(&self.e) }
    fn fold_back(&self) -> &Op::Acc { self.back.last().unwrap_or(&self.e) }
    pub fn fold(&self) -> Op::Acc { Op::prod_acc(&self.fold_front(), &self.fold_back()) }
    
    pub fn push_front(&mut self, v: Op::Value) {
        self.front.push(Op::prod_acc(&Op::to_acc(&v), self.fold_front()));
        self.deque.push_front(v);
    }
    
    pub fn push_back(&mut self, v: Op::Value) {
        self.back.push(Op::prod_acc(self.fold_back(), &Op::to_acc(&v)));
        self.deque.push_back(v);
    }
    
    pub fn pop_front(&mut self) -> Option<Op::Value> {
        let res = self.deque.pop_front();
        if self.front.pop().is_none() { self.rebuild(); }
        res
    }
    
    pub fn pop_back(&mut self) -> Option<Op::Value> {
        let res = self.deque.pop_back();
        if self.back.pop().is_none() { self.rebuild(); }
        res
    }
    
    fn rebuild(&mut self) {
        self.front.clear(); self.back.clear();
        let len = self.deque.len();
        let mut tmp = Op::id_acc();
        for i in (0..len/2).rev() { tmp = Op::prod_acc(&Op::to_acc(&self.deque[i]), &tmp); self.front.push(tmp.clone()); }
        tmp = Op::id_acc();
        for i in len/2..len { tmp = Op::prod_acc(&tmp, &Op::to_acc(&self.deque[i])); self.back.push(tmp.clone()); }
    }
}

impl<Op: FoldableDequeOp> FromIterator<Op::Value> for FoldableDeque<Op> {
    fn from_iter<T: IntoIterator<Item = Op::Value>>(iter: T) -> Self {
        let mut deq = Self::new();
        for v in iter { deq.push_back(v); }
        deq
    }
}

impl<Op: FoldableDequeOp> Clone for FoldableDeque<Op> where Op::Value: Clone {
    fn clone(&self) -> Self {
        Self { deque: self.deque.clone(), front: self.front.clone(), back: self.back.clone(), e: self.e.clone() }
    }
}

impl<Op: FoldableDequeOp> Debug for FoldableDeque<Op> where Op::Value: Debug, Op::Acc: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} (fold: {:?})", self.deque, self.fold())
    }
}
