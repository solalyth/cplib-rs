use std::ops::Index;

#[derive(Debug)]
pub struct CSR {
    dat: Vec<usize>,
    idx: Vec<usize>,
}

impl CSR {
    pub fn new(len: usize, dat: &[(usize, usize)]) -> Self {
        let mut idx = vec![0; len+1];
        for &(i, _) in dat {
            if i+2 <= len { idx[i+2] += 1; }
        }
        for i in 0..len { idx[i+1] += idx[i]; }
        let mut res = vec![0; dat.len()];
        for &(i, v) in dat { res[idx[i+1]] = v; idx[i+1] += 1; }
        Self { dat: res, idx }
    }
    
    pub fn new_undirected(len: usize, dat: &[(usize, usize)]) -> Self {
        let mut edges = vec![];
        for &(i, j) in dat {
            edges.push((i, j));
            edges.push((j, i));
        }
        Self::new(len, &edges)
    }
    
    pub fn from_iter(len: usize, iter: impl IntoIterator<Item = (usize, usize)>) -> Self {
        let v = iter.into_iter().collect::<Vec<_>>();
        Self::new(len, &v)
    }
    
    pub fn from_iter_undirected(len: usize, iter: impl IntoIterator<Item = (usize, usize)>) -> Self {
        let mut v = iter.into_iter().collect::<Vec<_>>();
        for i in 0..v.len() { v.push((v[i].1, v[i].0)); }
        Self::new(len, &v)
    }
    
    pub fn len(&self) -> usize { self.idx.len()-1 }
    
    pub fn sort(&mut self) {
        for i in 0..self.len() {
            self.dat[self.idx[i]..self.idx[i+1]].sort_unstable();
        }
    }
    
    /// sorted である必要がある。
    pub fn contains(&self, u: usize, v: usize) -> bool {
        debug_assert!(self[u].is_sorted());
        self[u].binary_search(&v).is_ok()
    }
}

impl Index<usize> for CSR {
    type Output = [usize];
    fn index(&self, i: usize) -> &Self::Output {
        &self.dat[self.idx[i]..self.idx[i+1]]
    }
}

impl<'a> IntoIterator for &'a CSR {
    type Item = (usize, usize);
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter { Iter { src: self, cur: 0, i: 0 } }
}



pub struct Iter<'a> {
    src: &'a CSR,
    cur: usize,
    i: usize
}

impl<'a> Iterator for Iter<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let mut res = None;
        if self.cur < self.src.dat.len() {
            while self.src.idx[self.i+1] == self.cur { self.i += 1; }
            res = Some((self.i, self.src.dat[self.cur]));
        }
        self.cur += 1;
        res
    }
}
