/// Operator for [`DST`]
pub trait DSTOp: Sized {
    type Value: Clone;
    
    /// `Value` の単位元を返す。
    fn id_value() -> Self::Value;
    
    /// `Value` の積を返す。
    fn prod_value(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    
    fn dst_new(base: Vec<Self::Value>) -> DST<Self> { DST::new(base) }
    fn dst_from_iter(iter: impl IntoIterator<Item = Self::Value>) -> DST<Self> { DST::new(iter.into_iter().collect()) }
}

/// Disjoint Sparse Table
pub struct DST<Op: DSTOp> {
    table: Vec<Vec<Op::Value>>,
    dep: usize,
}

impl<Op: DSTOp> DST<Op> {
    pub fn new(mut base: Vec<Op::Value>) -> Self {
        let dep = (base.len().max(2)-1).ilog2() as usize + 1; // ceil(log2)
        base.resize(1<<dep, Op::id_value());
        
        let mut table = vec![];
        for i in 1..dep {
            let mut v = vec![Op::id_value(); 1<<dep];
            for w in (0..1<<dep).step_by(1<<i+1) {
                let (mut l, mut r) = (w^(1<<i), w^(1<<i));
                for _ in 0..1<<i {
                    (v[l-1], v[r]) = (Op::prod_value(&base[l-1], &v[l]), Op::prod_value(&v[r-1], &base[r]));
                    l -= 1;
                    r += 1;
                }
            }
            table.push(v);
        }
        table.insert(0, base);

        Self { table, dep }
    }
    
    pub fn len(&self) -> usize {
        1<<self.dep
    }
    
    pub fn fold(&self, l: usize, r: usize) -> Op::Value {
        if l >= r { return Op::id_value(); }
        if l+1 == r { return self.table[0][l].clone(); }
        let msb = (l ^ (r-1)).ilog2() as usize;
        Op::prod_value(&self.table[msb][l], &self.table[msb][r-1])
    }
}
