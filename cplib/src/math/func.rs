pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 { (a, b) = (b, a%b); }
    a
}

pub fn lcm(a: usize, b: usize) -> Option<usize> {
    (a/gcd(a, b)).checked_mul(b)
}

/// `ax + by = gcd(a,b)` を満たす `(x, y, gcd(a, b))` を返す。
/// 
/// - `a == 0 && b == 0` のとき `(0, 0, 0)` を返す。
/// - `a == 0` のとき `(0, sgn(b), |b|)` を返し、`b == 0` のとき `(sgn(a), 0, |a|)` を返す。
/// - そうでないとき、`|x| <= |b|/g` かつ `|y| <= |a|/g` を満たす。
pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut p0, mut q0, mut r0, mut p1, mut q1, mut r1) = (a.signum(), 0, a.abs(), 0, b.signum(), b.abs());
    while r1 != 0 {
        let t = r0/r1;
        (p0, q0, r0, p1, q1, r1) = (p1, q1, r1, p0 - t*p1, q0 - t*q1, r0 - t*r1);
    }
    (p0, q0, r0)
}


/// ベズーの等式 `ax + by = c` を満たす解 `(x, y, dx, dy)` を返す。ただし、`0 <= x < |b|/g` を満たし、`0 < dx` である。
/// 
/// # Panics
/// 
/// - if `b == 0`
pub fn bezout(a: i64, b: i64, c: i64) -> Option<(i64, i64, i64, i64)> {
    assert!(b != 0);
    let (x, y, g) = extgcd(a, b);
    if c%g != 0 { return None; }
    let t = (c/g*x).div_euclid(b/g);
    Some((c/g*x - t*(b/g), c/g*y + t*(a/g), b.abs()/g, -a*b.signum()/g))
}



pub fn modinv(x: usize, m: usize) -> Option<usize> {
    let (y, _, g) = extgcd(x as i64, m as i64);
    if g == 1 { Some(y.rem_euclid(m as i64) as usize) } else { None }
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
/// `x^n = x^{phi(m) * [phi(m) <= n] + (n mod \phi(m))} (mod m)` が成り立つ。
pub fn modpow(mut x: u128, mut n: u128, m: u128) -> u128 {
    if m == 1 { return 0; }
    if n == 0 { return 1; }
    x %= m;
    let mut res = 1;
    while n != 0 {
        if n&1 == 1 { res = res*x % m; }
        x = x*x % m;
        n /= 2;
    }
    res
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
        let g = gcd(p.abs() as usize, q.abs() as usize) as i128;
        (p/g, q/g)
    } else {
        (1, 0)
    }
}

/// 傾き `p/q` の uv 直交座標系に変換する。`[qx+py, -px+qy]` を返す。必要ならば事前に [`rational`] を取ること。
pub fn into_uv([x, y]: [i128; 2], p: i128, q: i128) -> [i128; 2] {
    [q*x+p*y, q*y-p*x]
}



/// `(p, exp)` の列を受け取って、約数を返す。昇順かは保証されていない。
/// 
/// 約数の個数は `N^(1/3)` 個程度である。(ref. [競プロにおける約数の個数の見積もり - noshi91](https://noshi91.hatenablog.com/entry/2022/07/05/021040))
pub fn divisors(pe: &[(usize, usize)]) -> Vec<usize> {
    let mut res = vec![1];
    for &(p, e) in pe.into_iter() {
        for i in 0..res.len() {
            let mut k = res[i];
            for _ in 0..e { k *= p; res.push(k); }
        }
    }
    res
}
