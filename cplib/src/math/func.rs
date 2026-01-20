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
pub fn mpow(mut x: u128, mut n: u128, m: u128) -> u128 {
    x %= m;
    if n == 0 { return if x == 0 {0} else {1}; }
    let mut res = 1;
    while n != 0 {
        if n&1 == 1 { res = res*x % m; }
        x = x*x % m;
        n >>= 1;
    }
    res
}



/// 和が `s` である、長さ `n` の非負整数列を辞書順に返す。`res.len() == (s+n-1)! / (s! * (n-1)!)`
pub fn partitions(n: usize, s: usize) -> Option<Vec<Vec<usize>>> {
    if n == 0 && s != 0 { return None; }
    if s == 0 { return Some(vec![vec![0; n]]); }
    
    let mut cur = vec![0; n];
    cur[n-1] = s;
    let (mut res, mut t) = (vec![cur.clone()], n-1);
    
    while t != 0 {
        cur[t-1] += 1;
        cur[n-1] = std::mem::take(&mut cur[t])-1;
        if cur[n-1] == 0 { t -= 1; } else { t = n-1; }
        res.push(cur.clone());
    }
    
    Some(res)
}




pub fn into_ary(mut n: usize, base: usize) -> Vec<usize> {
    let mut res = vec![];
    while n != 0 {
        res.push(n%base); n /= base;
    }
    res
}

pub fn from_ary(d: &[usize], base: usize) -> usize {
    let mut res = 0;
    for &d in d { res = res*base + d; }
    res
}

pub fn digit_ary(mut n: usize, base: usize) -> Option<usize> {
    assert!(2 <= base);
    if n == 0 { return None; }
    let mut cnt = 0;
    while n != 0 { n /= base; cnt += 1; }
    Some(cnt)
}
