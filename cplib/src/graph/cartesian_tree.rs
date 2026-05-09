pub struct CartesianTree {
    pub l: Vec<usize>,
    pub r: Vec<usize>,
    pub order_inv: Vec<usize>,
}

impl CartesianTree {
    /// 小さい方が子
    pub fn new<T: PartialOrd>(rank: &[T]) -> Self {
        let n = rank.len();
        let (mut l, mut r, mut stk, mut order_inv) = (vec![!0; n], vec![!0; n], vec![0], vec![]);
        for i in 1..n {
            while let Some(j) = stk.pop() {
                if rank[j] < rank[i] {
                    r[j] = i;
                    stk.push(j);
                    break;
                } else {
                    l[i] = j;
                    order_inv.push(j);
                }
            }
            stk.push(i);
        }
        for i in stk.into_iter().rev() { order_inv.push(i); }
        
        CartesianTree { l, r, order_inv }
    }
    
    pub fn child(&self, i: usize) -> [usize; 2] { [self.l[i], self.r[i]] }
}
