use std::collections::BinaryHeap;

#[derive(Clone)]
pub struct PriorityQueue<T: Clone + Ord> {
    dat: BinaryHeap<T>,
    del: BinaryHeap<T>,
}

impl<T: Clone + Ord> PriorityQueue<T> {
    pub fn new() -> Self {
        Self { dat: BinaryHeap::new(), del: BinaryHeap::new() }
    }
    
    pub fn push(&mut self, x: T) { self.dat.push(x); }
    pub fn remove(&mut self, x: T) { self.del.push(x); }
    pub fn peek(&mut self) -> Option<&T> { self.calc(); self.dat.peek() }
    pub fn pop(&mut self) -> Option<T> { self.calc(); self.dat.pop() }
    pub fn len(&mut self) -> usize { self.dat.len() - self.del.len() }
    
    fn calc(&mut self) {
        while let Some(e) = self.dat.peek() {
            if self.del.peek() == Some(e) { self.dat.pop(); self.del.pop(); } else { break; }
        }
    }
}

use std::fmt::*;
impl<T: Clone + Ord + Debug> Debug for PriorityQueue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let Self { mut dat, mut del } = self.clone();
        let mut s = vec![];
        while let Some(e) = dat.pop() {
            if del.peek() == Some(&e) { del.pop(); } else { s.push(e); }
        }
        write!(f, "{s:?}")
    }
}
