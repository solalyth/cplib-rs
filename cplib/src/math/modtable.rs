static mut P: usize = 0;
static mut INV: Vec<usize> = vec![];
static mut F: Vec<usize> = vec![];
static mut FINV: Vec<usize> = vec![];

pub fn init(p: usize, max: usize) {
    let (mut inv, mut f, mut finv) = (vec![0; max+1], vec![0; max+1], vec![0; max+1]);
    // inv
    inv[1] = 1;
    for i in 2..=max {
        inv[i] = p - (p/i * inv[p%i] % p);
    }
    debug_assert!((1..=max).all(|i| i*inv[i]%p == 1));
    
    // f, finv
    f[0] = 1; finv[0] = 1;
    for i in 1..=max {
        f[i] = f[i-1]*i % p;
        finv[i] = finv[i-1]*inv[i] % p;
    }
    debug_assert!((0..=max).all(|i| f[i]*finv[i]%p == 1));

    unsafe { (P, INV, F, FINV) = (p, inv, f, finv); }
}

pub fn c(n: usize, k: usize) -> usize {
    if n < k { 0 } else { unsafe { F[n] * FINV[k] % P * FINV[n-k] % P } }
}

pub fn p(n: usize, k: usize) -> usize {
    if n < k { 0 } else { unsafe { F[n] * FINV[n-k] % P } }
}

pub fn f(n: usize) -> usize { unsafe { F[n] } }

pub fn finv(n: usize) -> usize { unsafe { FINV[n] } }
