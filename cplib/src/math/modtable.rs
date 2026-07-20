static mut P: usize = 0;
static mut INV: Vec<usize> = vec![];
static mut F: Vec<usize> = vec![];
static mut FINV: Vec<usize> = vec![];

macro_rules! a { ($n:expr, $x:expr) => {{ _O::calc($n); unsafe {$x} }} }

pub const O: _O = _O;

pub struct _O;
#[allow(non_snake_case, static_mut_refs)]
impl _O {
    pub fn init(&self, p: usize) {
        unsafe {
            if p == P { return; }
            (P, INV, F, FINV) = (p, vec![0, 1], vec![1, 1], vec![1, 1]);
        }
        _O::calc(1000);
    }
    
    fn calc(mut n: usize) {
        unsafe {
            let len = F.len();
            if n < len { return; }
            assert!(n < P);
            n = n.max(len*2)+1;
            
            for i in len..n {
                INV.push(P - (P/i*INV[P%i] % P));
                F.push(F[i-1]*i % P);
                FINV.push(FINV[i-1]*INV[i] % P);
            }
        }
    }
    
    // pub fn MC<const N: usize>(&self, k: [usize; N]) -> usize {
    //     let s = k.iter().sum::<usize>();
    //     _O::calc(s);
    //     unsafe {
    //         let mut res = F[s];
    //         for k in k { res = res * FINV[k] % P; }
    //         res
    //     }
    // }
    
    pub fn P(&self, n: usize, k: usize) -> usize {
        if n < k { 0 } else { a!(n, F[n]*FINV[n-k]%P) }
    }
    
    pub fn C(&self, n: usize, k: usize) -> usize {
        if n < k { 0 } else { a!(n, F[n]*FINV[k]%P*FINV[n-k]%P) }
    }
    
    pub fn MC<const N: usize>(&self, k: [usize; N]) -> usize {
        let s = k.iter().sum::<usize>();
        a!(s, k.iter().fold(F[s], |t, &k| t*FINV[k]%P))
    }
    
    /// `n` 個のボールを `col` 色に塗る方法は重複組合せ `H(col, n) = [x^n] (1-x)^{-col}` と等しい。
    pub fn color(&self, n: usize, col: usize) -> usize {
        if n == 0 { 1 } else { O.C(col+n-1, n) }
    }
    
    // pub fn C_naive(&self, n: usize, mut k: usize) -> usize {
    //     if n < k { return 0; }
    //     k = k.min(n-k);
    //     _O::calc(k);
    //     unsafe {
    //         let mut t = FINV[k];
    //         for i in n-k+1..=n { t = t*i % P; }
    //         t
    //     }
    // }
    
    pub fn F(&self, n: usize) -> usize { a!(n, F[n]) }
    pub fn FINV(&self, n: usize) -> usize { a!(n, FINV[n]) }
    pub fn INV(&self, n: usize) -> usize { a!(n, INV[n]) }
    pub fn MOD(&self) -> usize { unsafe { P } }
}
