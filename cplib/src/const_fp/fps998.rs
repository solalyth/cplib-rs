use crate::cplib::const_fp::fp::Fp998 as Fp;

static mut R: Vec<Fp> = vec![];

#[allow(static_mut_refs)]
fn init() {
    unsafe {
        let mut cur = Fp::raw(1);
        for _ in 0..=1<<23 {
            R.push(cur);
            cur = cur * Fp::raw(3).pow(998244352>>23);
        }
    }
}


pub trait FPS: AsRef<[Fp]> {
    fn fps_mul(&self, f: impl AsRef<[Fp]>) -> Vec<Fp> { mul(self.as_ref(), f.as_ref()) }
    fn fps_inv(&self, n: usize) -> Vec<Fp> { let w = self.as_ref(); pre(inv(w, log_ceil(n)), n) }
    
    fn fps_pow_naive(&self, mut exp: usize) -> Vec<Fp> {
        let mut res = vec![Fp::new(1)];
        let mut cur = self.as_ref().to_vec();
        while exp != 0 {
            if exp&1 == 1 { res = res.fps_mul(&cur); }
            cur = cur.fps_mul(&cur);
            exp >>= 1;
        }
        res
    }
}

impl<T: AsRef<[Fp]>> FPS for T {}

pub fn fps_mul_all(mut fl: Vec<Vec<Fp>>) -> Vec<Fp> {
    if fl.is_empty() { return vec![Fp::raw(1)]; }
    let mut map = std::collections::BTreeMap::new();
    while let Some(f) = fl.pop() { map.insert((f.len(), map.len()), f); }
    loop {
        let ((_, i), mut f) = map.pop_first().unwrap();
        let Some((_, g)) = map.pop_first() else { return f; };
        f = f.fps_mul(g);
        map.insert((f.len(), i), f);
    }
}





/// サイズ `2^log` の DFT を計算する。
fn fft(f: &[Fp], log: usize, inv: bool) -> Vec<Fp> {
    let mut res = vec![Fp::raw(0); 1<<log];
    for i in 0..f.len().min(1<<log) { res[i] = f[i]; }
    fft_inplace(&mut res, log, inv);
    res
}

fn fft_inplace(f: &mut [Fp], log: usize, inv: bool) {
    assert!(f.len() == 1<<log);
    if log == 0 { return; }
    for i in 0..1<<log { let j = bitrev(i, log); if i < j { f.swap(i, j); } }
    
    #[allow(static_mut_refs)]
    if unsafe{R.is_empty()} { init(); }
    
    for d in 0..log {
        for w in f.chunks_exact_mut(1<<d+1) {
            let (mut p, r) = if !inv {(0, 1<<23-d-1)} else {(1<<23, (1usize<<23-d-1).wrapping_neg())};
            for i in 0..1<<d {
                let j = i+(1<<d);
                let t = w[j]*unsafe{R[p]};
                (w[i], w[j], p) = (w[i]+t, w[i]-t, p.wrapping_add(r));
            }
        }
        // let mut r = Fp::raw(3).pow(998244352>>d+1);
        // if inv { r = r.inv(); }
        // for w in f.chunks_exact_mut(1<<d+1) {
        //     let mut p = Fp::raw(1);
        //     for i in 0..1<<d {
        //         let j = i+(1<<d);
        //         let t = w[j]*p;
        //         (w[i], w[j], p) = (w[i]+t, w[i]-t, p*r);
        //     }
        // }
    }
    
    if inv {
        let t = Fp::new(1<<log).inv();
        for x in f { *x = *x * t; }
    }
}

fn mul(f: &[Fp], g: &[Fp]) -> Vec<Fp> {
    let n = f.len() + g.len() - 1;
    if f.len().min(g.len()) < 64 {
        let mut res = vec![Fp::new(0); n];
        for i in 0..f.len() {
            for j in 0..g.len() {
                res[i+j] = res[i+j] + f[i]*g[j];
            }
        }
        res
    } else {
        let log = log_ceil(n);
        let (mut f, g) = (fft(&f, log, false), fft(&g, log, false));
        for i in 0..1<<log { f[i] = f[i] * g[i]; }
        fft_inplace(&mut f, log, true);
        pre(f, n)
    }
}



/// `f[0] != 0`
fn inv(f: &[Fp], log: usize) -> Vec<Fp> {
    assert!(f[0] != Fp::new(0));
    let mut cur = vec![f[0].inv()];
    for d in 0..log {
        let mut f = fft(&f[..f.len().min(1<<d+1)], d+1, false);
        let g = fft(&cur[..1<<d], d+1, false);
        for i in 0..1<<d+1 { f[i] = f[i] * g[i]; }
        fft_inplace(&mut f, d+1, true);
        for i in 0..1<<d { f[i] = Fp::new(0); }
        fft_inplace(&mut f, d+1, false);
        for i in 0..1<<d+1 { f[i] = f[i] * g[i]; }
        fft_inplace(&mut f, d+1, true);
        for i in 0..1<<d { f[i] = f[i] + cur[i]; }
        for x in &mut f { *x = -*x; }
        cur = f;
    }
    
    cur
}



pub fn bostan_mori(mut f: Vec<Fp>, mut g: Vec<Fp>, mut n: usize) -> Fp {
    let mut h = vec![Fp::new(0); g.len()];
    while n != 0 {
        for i in 0..g.len() { h[i] = if i%2 == 0 { g[i] } else { -g[i] }; }
        f = mul(&f, &h);
        g = mul(&g, &h);
        if n%2 == 0 {
            let k = (f.len()+1)/2;
            for i in 0..k { f[i] = f[i*2]; }
            f.truncate(k);
        } else {
            let k = f.len()/2;
            for i in 0..k { f[i] = f[i*2+1]; }
            f.truncate(k);
        }
        for i in 0..=g.len()/2 { g[i] = g[i*2]; }
        g.truncate(g.len()/2+1);
        n /= 2;
    }
    f[0] * g[0].inv()
}


fn bitrev(x: usize, log: usize) -> usize { x.reverse_bits() as usize >> (64-log) }
fn log_ceil(x: usize) -> usize { assert!(x != 0); (x-1).ilog2() as usize + 1 }
fn pre(mut f: Vec<Fp>, n: usize) -> Vec<Fp> { f.truncate(n); f }
