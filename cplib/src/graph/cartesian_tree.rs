#[derive(Debug)]
pub struct CartesianTree {
    pub l: Vec<usize>,
    pub r: Vec<usize>,
    pub order: Vec<usize>,
}

impl CartesianTree {
    pub fn new<T: PartialOrd>(rank: &[T], root_min: bool) -> Self {
        let n = rank.len();
        let (mut l, mut r, mut stk, mut order) = (vec![!0; n], vec![!0; n], vec![0], vec![]);
        for i in 1..n {
            while let Some(j) = stk.pop() {
                if (rank[j] < rank[i]) == root_min {
                    r[j] = i;
                    stk.push(j);
                    break;
                } else {
                    l[i] = j;
                    order.push(j);
                }
            }
            stk.push(i);
        }
        for i in stk.into_iter().rev() { order.push(i); }
        order.reverse();
        
        CartesianTree { l, r, order }
    }
    
    pub fn child(&self, i: usize) -> [usize; 2] { [self.l[i], self.r[i]] }
    
    pub fn range(&self) -> [Vec<usize>; 2] {
        let n = self.l.len();
        let (mut l, mut r) = (vec![0; n], vec![0; n]);
        for &i in self.order.iter().rev() {
            l[i] = i; r[i] = i+1;
            if self.l[i] != !0 { l[i] = l[self.l[i]]; }
            if self.r[i] != !0 { r[i] = r[self.r[i]]; }
        }
        [l, r]
    }
}
