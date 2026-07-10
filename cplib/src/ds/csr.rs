use std::{fmt::Debug, ops::{Index, IndexMut}};


#[derive(Clone)]
pub struct CSR<T> {
    dat: Vec<T>,
    idx: Vec<usize>,
}

pub type Edge = CSR<usize>;

impl Edge {
    /// 無向辺なら `und = true` とすること。
    pub fn from_edges(n: usize, und: bool, rev: bool, uv: &[(usize, usize)]) -> Self {
        if rev {
            Self::from_edges_iter(n, und, uv.iter().map(|e| (e.1, e.0)))
        } else {
            Self::from_edges_iter(n, und, uv.iter().cloned())
        }
    }

    /// 無向辺なら `und = true` とすること。
    pub fn from_edges_iter(n: usize, und: bool, iter: impl IntoIterator<Item = (usize, usize)> + Clone) -> Self {
        let mut idx = vec![0; n+2];
        for (i, j) in iter.clone() {
            idx[i+2] += 1;
            if und { idx[j+2] += 1; }
        }
        for i in 0..=n { idx[i+1] += idx[i]; }
        
        let mut dat = vec![0; idx.pop().unwrap()];
        for (i, j) in iter.into_iter() {
            dat[idx[i+1]] = j; idx[i+1] += 1;
            if und && i != j { dat[idx[j+1]] = i; idx[j+1] += 1; }
        }
        
        Self { dat, idx }
    }
}

impl<T> CSR<T> {
    /// `[]` に相当する配列を作る。`[[]]` ではないのでまず [`CSR::next_vec`] する必要があることに注意。
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
    
    pub fn sort(&mut self) where T: Ord {
        for i in 0..self.idx_len() {
            self.dat[self.idx[i]..self.idx[i+1]].sort_unstable();
        }
    }
    
    pub fn contains(&self, u: usize, x: &T) -> bool where T: Ord {
        self[u].binary_search(x).is_ok()
    }
}

impl<T> Index<usize> for CSR<T> {
    type Output = [T];
    fn index(&self, i: usize) -> &Self::Output {
        &self.dat[self.idx[i]..self.idx[i+1]]
    }
}

impl<T> IndexMut<usize> for CSR<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.dat[self.idx[i]..self.idx[i+1]]
    }
}

impl<T: Clone + Debug> Debug for CSR<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 0..self.idx_len() {
            s += &format!("{i}: [, ");
            for x in &self[i] {
                s.pop(); s.pop();
                s += &format!("{x:?}, ");
            }
            s.pop(); s.pop(); s += "], ";
        }
        s.pop(); s.pop();
        write!(f, "{{ {s} }}")
    }
}
