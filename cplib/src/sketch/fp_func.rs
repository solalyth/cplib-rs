pub fn mod_pow(mut x: usize, mut exp: usize, m: usize) -> usize {
    let mut res = 1;
    while exp != 0 {
        if exp&1 == 1 { res = res*x % m; }
        x = x*x % m;
        exp >>= 1;
    }
    res
}

pub fn primitive_root(p: usize, pe: &[(usize, usize)]) -> usize {
    if p == 2 { return 1; }
    if p == 998244353 { return 3; }
    for g in 2.. {
        if pe.iter().all(|&(x, _)| mod_pow(g, (p-1)/x, p) != 1) { return g; }
    }
    unreachable!()
}


use std::collections::HashMap;

/// fp log を `<O(p/B), O(B)>` で計算する。`B = sqrt(p/Q)` 程度にすると全体で `O(sqrt(pQ))` となる。
pub struct FpLog<const B: usize> {
    p: usize,
    gs: usize,
    table: HashMap<usize, usize>
}

impl<const B: usize> FpLog<B> {
    pub fn new(p: usize, g: usize) -> Self {
        let mut table = HashMap::new();
        let mut cur = 1;
        for i in 0..(p+B-2)/B {
            table.insert(cur, i);
            cur = cur*g%p;
        }
        
        Self { p, gs: mod_pow(g, (p+B-2)/B, p), table }
    }
    
    pub fn log(&self, mut x: usize) -> usize {
        assert!(x != 0);
        for i in 1..=B {
            x = x*self.gs % self.p;
            if let Some(&j) = self.table.get(&x) {
                return j*B - i;
            }
        }
        unreachable!()
    }
}
