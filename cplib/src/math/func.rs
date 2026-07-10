/// 非負値に対して `gcd(a: T, b: T) -> T` を計算する。
#[macro_export]
macro_rules! gcd {
    ($a:expr, $b:expr) => {{
        let (mut a, mut b) = ($a, $b);
        while b != 0 { (a, b) = (b, a%b); }
        a
    }};
    ($($x:expr),+) => {{
        let mut res = 0;
        $( res = gcd!(res, $x); )*
        res
    }}
}

/// 非負値に対して `lcm(a: T, b: T) -> T` を計算する。表現できない場合は saturating する。
#[macro_export]
macro_rules! lcm {
    ($a:expr, $b:expr) => {{
        let (a, b) = ($a, $b);
        if (a, b) == (0, 0) { 0 } else { (a/gcd!(a, b)).saturating_mul(b) }
    }};
}



/// `ax + by = gcd(a,b)` を満たす `(x, y, gcd(a, b))` を返す。
/// 
/// - `a == 0 && b == 0` のとき `(0, 0, 0)` を返す。
/// - `a == 0` のとき `(0, sgn(b), |b|)` を返し、`b == 0` のとき `(sgn(a), 0, |a|)` を返す。
/// - そうでないとき、`|x| <= |b|/g` かつ `|y| <= |a|/g` を満たす。
#[macro_export]
macro_rules! extgcd {
    ($a:expr, $b:expr) => {{
        let (a, b) = ($a, $b);
        let (mut p0, mut q0, mut r0, mut p1, mut q1, mut r1) = (a.signum(), 0, a.abs(), 0, b.signum(), b.abs());
        while r1 != 0 {
            let t = r0/r1;
            (p0, q0, r0, p1, q1, r1) = (p1, q1, r1, p0 - t*p1, q0 - t*q1, r0 - t*r1);
        }
        // assert!(0 <= r0 && (p0.abs() * r0 <= b.abs() && q0.abs() * r0 <= a.abs()));
        (p0, q0, r0)
    }};
}

/// `ax + by = c` を満たす解 `(x, y, dx, dy)` を返す。ただし、`0 <= x < |b|/g` を満たし、`0 < dx` である。
/// 
/// # Constraints
/// 
/// - if `b == 0`
/// - if not `|a|, |b| < 3.03e9 < sqrt(i64::MAX)`
/// 
/// # Verify
/// 
/// - https://atcoder.jp/contests/abc315/submissions/76183752 `|a|, |b| <= 1e9`
pub fn bezout(mut a: i64, mut b: i64, mut c: i64) -> Option<(i64, i64, i64, i64)> {
    assert!(b != 0 && a.abs().max(b.abs()) < 3.03e9 as i64);
    let (x, _, g) = extgcd!(a, b);
    if c%g != 0 { return None; }
    (a, b, c) = (a/g, b/g, c/g);
    
    let x = (c%b*x).rem_euclid(b);
    Some((x, (c-a*x)/b, b.abs(), -a*b.signum()))
}




// pub fn crt((a1, m1): (usize, usize), (a2, m2): (usize, usize)) -> Option<(usize, usize)> {
//     let g = gcd(m1, m2);
//     let (a1, a2) = ((a1%g) as i64, (a2%g) as i64);
//     if (a2-a1)%g as i64 != 0 { return None; }
//     let a1inv = modinv(m1/g, m2/g)?;
    
//     todo!()
// }



/// `x^n mod m` を計算する。`0^0 == 1` とする。
/// 
/// `x^n = x^{phi(m)*[phi(m) <= n] + (n mod \phi(m))} (mod m)` が成り立つ。
#[macro_export]
macro_rules! modpow {
    ($x:expr, $n:expr, $m:expr) => {{
        let (mut x, mut n, m) = ($x, $n, $m);
        x %= m;
        let mut res = 1;
        while n != 0 {
            if n & 1 == 1 { res = res * x % m; }
            x = x * x % m;
            n /= 2;
        }
        res%m
    }};
}



/// 和が `s` である、長さ `n` の非負整数列を辞書順に返す。`res.len() == (s+n-1)! / (s! * (n-1)!)`
// pub fn partitions(n: usize, s: usize) -> Option<Vec<Vec<usize>>> {
//     if n == 0 && s != 0 { return None; }
//     if s == 0 { return Some(vec![vec![0; n]]); }
    
//     let mut cur = vec![0; n];
//     cur[n-1] = s;
//     let (mut res, mut t) = (vec![cur.clone()], n-1);
    
//     while t != 0 {
//         cur[t-1] += 1;
//         cur[n-1] = std::mem::take(&mut cur[t])-1;
//         if cur[n-1] == 0 { t -= 1; } else { t = n-1; }
//         res.push(cur.clone());
//     }
    
//     Some(res)
// }




pub fn into_ary(mut n: u64, base: u64) -> Vec<u64> {
    let mut res = vec![];
    while n != 0 { res.push(n%base); n /= base; }
    res
}

pub fn from_ary(d: &[u64], base: u64) -> u64 {
    let mut res = 0;
    for &d in d { res = res*base + d; }
    res
}

// pub fn digit_ary(mut n: usize, base: usize) -> usize {
//     assert!(2 <= base);
//     let mut cnt = 0;
//     while n != 0 { n /= base; cnt += 1; }
//     cnt
// }



/// 既約分数 `(x, y) == x/y` あるいは無限大 `(1, 0) == infty` を返す。ただし `x >= 0` を満たす。
/// 
/// # Panics
/// 
/// - if `(p, q) == (0, 0)`
pub fn rational(mut p: i128, mut q: i128) -> (i128, i128) {
    assert!((p, q) != (0, 0));
    if q != 0 {
        if q < 0 { (p, q) = (-p, -q); }
        let g = gcd!(p.abs(), q.abs());
        (p/g, q/g)
    } else {
        (1, 0)
    }
}

/// 傾き `p/q` の uv 直交座標系に変換する。`[qx+py, -px+qy]` を返す。必要ならば事前に [`rational`] を取ること。
pub fn into_uv([x, y]: [i128; 2], p: i128, q: i128) -> [i128; 2] {
    [q*x+p*y, q*y-p*x]
}



// pub fn mex(mut v: Vec<usize>) -> usize {
//     v.sort_unstable(); v.dedup();
//     for i in 0..v.len() { if v[i] != i { return i; } }
//     v.len()
// }
