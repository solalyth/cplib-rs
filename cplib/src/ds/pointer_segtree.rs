trait PointerSegtreeOp {
    type Value: Clone;
    type Lazy: Clone;
    
    fn id_value() -> Self::Value;
    fn prod_value(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    fn act_value(value: &mut Self::Value, lazy: &Self::Lazy);
    fn prod_lazy(lazy: &mut Self::Lazy, ad: &Self::Lazy);
}



/// 疎永続遅延セグメント木
pub struct PointerSegtree<Op: PointerSegtreeOp> {
    depth: usize,
    pool: Vec<Node<Op>>,
}

impl<Op: PointerSegtreeOp> PointerSegtree<Op> {
    /// `root` must be `0`
    pub fn new(len: usize) -> Self {
        let depth = len.next_power_of_two().trailing_zeros() as usize;
        Self { depth, pool: vec![Node { value: Op::id_value(), lazy: None, c: [!0; 2] }] }
    }
    
    pub fn len(&self) -> usize { 1 << self.depth }
    
    pub fn push(&mut self, i: usize) {
        let lazy = std::mem::replace(&mut self.pool[i].lazy, None);
        for c in 0..2 {
            let cidx = self.pool[i].c[c];
            if cidx != !0 {
                if let Some(lazy) = &lazy {
                    let mut node = self.pool[cidx].clone();
                    Op::act_value(&mut node.value, lazy);
                    self.pool[i].c[c] = self.pool.len();
                    self.pool.push(node);
                }
            } else {
                let mut node = Node::id();
                if let Some(lazy) = &lazy {
                    Op::act_value(&mut node.value, lazy);
                    node.lazy = Some(lazy.clone());
                }
                self.pool[i].c[c] = self.pool.len();
                self.pool.push(node);
            }
        }
    }
    
    pub fn get(&mut self, mut root: usize, mut i: usize) -> &Op::Value {
        assert!(i < self.len());
        
        for d in (0..self.depth).rev() {
            self.push(root);
            if i < 1<<d {
                root = self.pool[root].c[0];
            } else {
                root = self.pool[root].c[1];
                i -= 1 << d;
            }
        }
        
        &self.pool[root].value
    }
    
    pub fn set(&mut self, mut root: usize, mut i: usize, f: impl FnOnce(&mut Op::Value)) -> usize {
        let head = self.pool.len();
        
    }
}




fn prod_lazy<Op: PointerSegtreeOp>(lazy: &mut Option<Op::Lazy>, ad: &Op::Lazy) {
    if let Some(lazy) = lazy {
        Op::prod_lazy(lazy, ad);
    } else {
        *lazy = Some(ad.clone());
    }
}

struct Node<Op: PointerSegtreeOp> {
    value: Op::Value,
    lazy: Option<Op::Lazy>,
    c: [usize; 2],
}

impl<Op: PointerSegtreeOp> Node<Op> {
    fn id() -> Self {
        Self { value: Op::id_value(), lazy: None, c: [!0; 2] }
    }
    fn clone(&self) -> Self {
        Self { value: self.value.clone(), lazy: self.lazy.clone(), c: self.c }
    }
}
