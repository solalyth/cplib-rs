static mut P: usize = 0;
static mut INV: Vec<usize> = vec![];
static mut F: Vec<usize> = vec![];
static mut FINV: Vec<usize> = vec![];

pub const O: _O = _O;

pub struct _O;
#[allow(non_snake_case, static_mut_refs)]
impl _O {
    pub fn init(&self, p: usize) {
        unsafe { P = p; INV.extend([0, 1]); F.extend([1, 1]); FINV.extend([1, 1]); }
        _O::calc(10000);
    }
    
    fn calc(mut n: usize) {
        unsafe {
            let len = F.len();
            if n < len { return; }
            n = n.max(len*2).min(P-1);
            assert!(P != 0 && 2 <= n && n < P);
            
            INV.resize(n+1, 0);
            F.resize(n+1, 0);
            FINV.resize(n+1, 0);
            
            // inv
            INV[1] = 1;
            for i in len..=n { INV[i] = P - (P/i * INV[P%i] % P); }
            
            // f, finv
            F[0] = 1; FINV[0] = 1;
            for i in len..=n {
                F[i] = F[i-1]*i % P;
                FINV[i] = FINV[i-1]*INV[i] % P;
            }
        }
    }
    
    pub fn MC<const N: usize>(&self, k: [usize; N]) -> usize {
        let s = k.iter().sum::<usize>();
        _O::calc(s);
        unsafe {
            let mut res = F[s];
            for k in k { res = res * FINV[k] % P; }
            res
        }
    }
    
    pub fn C(&self, n: usize, k: usize) -> usize {
        if n < k { 0 } else { _O::calc(n); unsafe { F[n] * FINV[k] % P * FINV[n-k] % P } }
    }
    
    pub fn C_naive(&self, n: usize, k: usize) -> usize {
        if n < k { return 0; }
        _O::calc(k);
        unsafe {
            let mut t = FINV[k];
            for i in n-k+1..=n { t = t*i % P; }
            t
        }
    }
    
    pub fn P(&self, n: usize, k: usize) -> usize {
        if n < k { 0 } else { _O::calc(n); unsafe { F[n] * FINV[n-k] % P } }
    }
    
    /// `k` 個のボールを `n` 色に塗る方法。`H(n, k) = [x^k] (1-x)^{-n} = M{n-1, k}`
    pub fn H(&self, n: usize, k: usize) -> usize { assert!(n != 0); self.C(n+k-1, k) }
    
    pub fn F(&self, n: usize) -> usize { unsafe { _O::calc(n); F[n] } }
    pub fn Finv(&self, n: usize) -> usize { unsafe { _O::calc(n); FINV[n] } }
    pub fn INV(&self, n: usize) -> usize { unsafe { _O::calc(n); INV[n] } }
}
