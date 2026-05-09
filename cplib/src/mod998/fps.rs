use crate::cplib::mod998::fp::Fp;


/// `ROOT[i]` には `1` の `2^(i+1)` 乗根が入っている。`15311432` は `2^23` 乗根の一つ。
const ROOT: [Fp; 23] = {
    let (mut pow, mut cur, mut i) = ([Fp::new(0); 23], Fp::new(15311432), 23);
    while i != 0 {
        i -= 1;
        pow[i] = cur;
        cur = cur.mul(cur);
    }
    pow
};

const ROOT_INV: [Fp; 23] = {
    let (mut pow, mut cur, mut i) = ([Fp::new(0); 23], Fp::new(15311432).inv(), 23);
    while i != 0 {
        i -= 1;
        pow[i] = cur;
        cur = cur.mul(cur);
    }
    pow
};


pub trait FPS: AsRef<[Fp]> {
    fn fps_mul(&self, other: impl AsRef<[Fp]>) -> Vec<Fp> { convolution(self.as_ref(), other.as_ref()) }
    fn fps_inv(&self) -> Vec<Fp> { inv(self.as_ref(), ilog2_ceil(self.as_ref().len())) }
    
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



/// サイズ `2^log` の DFT を計算する。
fn fft(f: &[Fp], log: usize, inv: bool) -> Vec<Fp> {
    if log == 0 { return vec![f[0]]; }
    
    let mut a = vec![Fp::new(0); 1<<log];
    for i in 0..f.len() { let idx = bitrev(i, log); a[idx] = a[idx] + f[i]; }
    
    for d in 0..log {
        for w in a.chunks_exact_mut(1<<d+1) {
            let mut p = Fp::new(1);
            for i in 0..1<<d {
                let t = w[i+(1<<d)] * p;
                w[i+(1<<d)] = w[i] - t;
                w[i] = w[i] + t;
                p = p.mul(if !inv {ROOT[d]} else {ROOT_INV[d]});
            }
        }
    }
    
    if inv {
        let t = Fp::new(1<<log).inv();
        for x in &mut a { *x = *x * t; }
    }
    
    a
}

fn convolution(f: &[Fp], g: &[Fp]) -> Vec<Fp> {
    let n = f.len() + g.len() - 1;
    if f.len().min(g.len()) <= 64 {
        let mut res = vec![Fp::new(0); n];
        for i in 0..f.len() {
            for j in 0..g.len() {
                res[i+j] = res[i+j] + f[i]*g[j];
            }
        }
        res
    } else {
        let log = ilog2_ceil(n);
        let (mut f, g) = (fft(&f, log, false), fft(&g, log, false));
        for i in 0..1<<log { f[i] = f[i] * g[i]; }
        let mut f = fft(&f, log, true);
        f.truncate(n);
        f
    }
}

/// `f[0] != 0`
fn inv(f: &[Fp], log: usize) -> Vec<Fp> {
    let mut cur = vec![Fp::new(0); 1<<log];
    cur[0] = f[0].inv();
    for d in 0..log {
        let mut f = fft(&f[..f.len().min(1<<d+1)], d+1, false);
        let g = fft(&cur[..1<<d], d+1, false);
        for i in 0..1<<d+1 { f[i] = f[i] * g[i]; }
        f = fft(&f, d+1, true);
        for i in 0..1<<d { f[i] = Fp::new(0); }
        f = fft(&f, d+1, false);
        for i in 0..1<<d+1 { f[i] = f[i] * g[i]; }
        f = fft(&f, d+1, true);
        for i in 1<<d..1<<d+1 { cur[i] = -f[i]; }
    }
    
    cur
}



pub fn bostan_mori(mut f: Vec<Fp>, mut g: Vec<Fp>, mut n: usize) -> Fp {
    let mut h = vec![Fp::new(0); g.len()];
    while n != 0 {
        for i in 0..g.len() { h[i] = if i%2 == 0 { g[i] } else { -g[i] }; }
        f = convolution(&f, &h);
        g = convolution(&g, &h);
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


const fn bitrev(x: usize, log: usize) -> usize { x.reverse_bits() as usize >> (64-log) }
const fn ilog2_ceil(x: usize) -> usize { if x == 1 { 0 } else { (x-1).ilog2() as usize + 1 } }
