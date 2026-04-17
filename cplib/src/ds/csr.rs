use std::ops::{Index, IndexMut};


#[derive(Debug)]
pub struct CSR<T: Default> {
    dat: Vec<T>,
    idx: Vec<usize>,
}

/// `(j, idx) in csr[i]` means `idx: i -> j`
pub type Edge = CSR<(usize, usize)>;

impl Edge {
    /// 無向辺なら `und = true` とすること。
    pub fn from_edges(n: usize, und: bool, iter: impl IntoIterator<Item = (usize, usize)> + Clone) -> Self {
        let mut idx = vec![0; n+2];
        for (i, j) in iter.clone() {
            idx[i+2] += 1;
            if und { idx[j+2] += 1; }
        }
        for i in 0..=n { idx[i+1] += idx[i]; }
        
        let mut dat = vec![(0, 0); idx.pop().unwrap()];
        for (k, (i, j)) in iter.into_iter().enumerate() {
            dat[idx[i+1]] = (j, k); idx[i+1] += 1;
            if und && i != j { dat[idx[j+1]] = (i, k); idx[j+1] += 1; }
        }
        
        Self { dat, idx }
    }
    
    pub fn sort(&mut self) {
        for i in 0..self.idx_len() {
            self.dat[self.idx[i]..self.idx[i+1]].sort_unstable();
        }
    }
    
    pub fn contains(&self, u: usize, v: usize) -> bool {
        self[u].binary_search_by_key(&v, |e| e.0).is_ok()
    }
}

impl<T: Default> CSR<T> {
    /// `[]` に相当する配列を作る。
    pub fn new() -> Self {
        Self { dat: vec![], idx: vec![0] }
    }
    
    pub fn idx_len(&self) -> usize { self.idx.len()-1 }
    pub fn dat_len(&self) -> usize { self.dat.len() }
    
    pub fn push(&mut self, x: T) {
        self.dat.push(x); *self.idx.last_mut().unwrap() += 1;
    }
    
    pub fn next_vec(&mut self) {
        self.idx.push(self.dat.len());
    }
}

impl<T: Default> Index<usize> for CSR<T> {
    type Output = [T];
    fn index(&self, i: usize) -> &Self::Output {
        &self.dat[self.idx[i]..self.idx[i+1]]
    }
}

impl<T: Default> IndexMut<usize> for CSR<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.dat[self.idx[i]..self.idx[i+1]]
    }
}
