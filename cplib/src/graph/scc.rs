use std::mem::replace;

pub struct Scc {
    n: usize,
    e: Vec<Vec<usize>>,
    inv: Vec<Vec<usize>>
}

impl Scc {
    pub fn new(n: usize) -> Self {
        Self { n, e: vec![vec![]; n], inv: vec![vec![]; n] }
    }
    
    pub fn add(&mut self, u: usize, v: usize) {
        self.e[u].push(v);
        self.inv[v].push(u);
    }
    
    pub fn calc(self) -> Vec<Vec<usize>> {
        let (mut t, mut seen, mut stk, mut res) = (Vec::with_capacity(self.n), vec![false; self.n], vec![], vec![]);
        for s in 0..self.n {
            if replace(&mut seen[s], true) { continue; }
            stk.push(s);
            while let Some(x) = stk.pop() {
                let i = x & (1<<32)-1;
                if let Some(&j) = self.e[i].get(x>>32) {
                    stk.push(x+(1<<32));
                    if !replace(&mut seen[j], true) { stk.push(j); }
                } else {
                    t.push(i);
                }
            }
        }
        
        for s in t.into_iter().rev() {
            if !replace(&mut seen[s], false) { continue; }
            let mut tmp = vec![s];
            for i in 0.. {
                let Some(&i) = tmp.get(i) else { break; };
                for &j in &self.inv[i] {
                    if replace(&mut seen[j], false) { tmp.push(j); }
                }
            }
            res.push(tmp);
        }
        
        res
    }
}
