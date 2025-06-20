#[derive(Debug)]
pub struct ModTable {
    p: usize,
    inv: Vec<usize>,
    f: Vec<usize>,
    finv: Vec<usize>,
}

impl ModTable {
    pub fn new_prime(max: usize, p: usize) -> Self {
        let (mut inv, mut f, mut finv) = (vec![0; max+1], vec![0; max+1], vec![0; max+1]);
        inv[1] = 1;
        for i in 2..=max {
            // let p = qi + r, i = -r/q, so inv[i] = -q/r
            inv[i] = p - (p/i * inv[p%i] % p);
        }
        
        f[0] = 1; finv[0] = 1;
        
        for i in 1..=max {
            f[i] = f[i-1]*i % p;
            finv[i] = finv[i-1]*inv[i] % p;
        }
        
        debug_assert!((1..=max).all(|i| i*inv[i]%p == 1));
        debug_assert!((0..=max).all(|i| f[i]*finv[i]%p == 1));
        
        Self { p, inv, f, finv }
    }
    
    pub fn max(&self) -> usize { self.inv.len()-1 }
    
    pub fn c(&self, n: usize, k: usize) -> usize {
        if n < k { 0 } else { self.f[n] * self.finv[k] % self.p * self.finv[n-k] % self.p }
    }
    
    pub fn p(&self, n: usize, k: usize) -> usize {
        if n < k { 0 } else { self.f[n] * self.finv[n-k] % self.p }
    }
    
    pub fn f(&self, n: usize) -> usize { self.f[n] }
}
