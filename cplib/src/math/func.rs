/// abs of `gcd(a, b)`
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 { (a, b) = (b, a%b); }
    a.abs()
}

/// `ax + by = gcd(a,b)` なる `(x, y, gcd(a, b))` を返す。ただし `|x| <= b, |y| <= a` を満たす。
pub fn extended_gcd(mut a: i64, mut b: i64) -> (i64, i64, i64) {
    let mut st = vec![];
    while b != 0 { st.push(a/b); (a, b) = (b, a%b); }
    let (mut x, mut y) = (a.signum(), 0);
    for z in st.into_iter().rev() { (x, y) = (y, x - z*y); }
    (x, y, a.abs())
}

/// `x^n mod m` を計算する。
pub fn mpow(mut x: usize, mut n: usize, m: usize) -> usize {
    x %= m;
    if x == 0 { return 0; }
    let mut res = 1;
    while n != 0 {
        if n&1 == 1 { res = res*x % m; }
        x = x*x % m;
        n >>= 1;
    }
    res
}



/// `ax^2 + bx + c = 0` の解を返す。
pub fn solve_quad(mut a: i128, mut b: i128, mut c: i128) -> Option<Vec<i128>> {
    fn isqrt(n: i128) -> i128 { (n as f64).sqrt() as i128 }
    let mut res = vec![];
    if a == 0 {
        if b == 0 && c == 0 { return None; }
        if b != 0 && c%b == 0 { res.push(-c/b); }
        return Some(res);
    }
    if a < 0 { (a, b, c) = (-a, -b, -c); }
    let d2 = b*b - 4*a*c;
    if d2 < 0 || isqrt(d2).pow(2) != d2 { return Some(res); }
    let d = isqrt(d2);
    if (-b-d) % (2*a) == 0 { res.push((-b-d)/(2*a)); }
    if (-b+d) % (2*a) == 0 { res.push((-b+d)/(2*a)); }
    Some(res)
}


/// `(a, b, c)`: `for i in a..=b, n/i == c`
/// 
/// # Memo
/// 
/// `n/i == j` iff `i in n/(j+1)+1..n/j+1` なので `n/j+1` を入れていけばよい。
/// `j = 1..=sqrt(n)` となる `i` はちょうど 1 つなので高速化させる。
pub fn quotient_floor(n: usize) -> Vec<(usize, usize, usize)> {
    let mut res = vec![];
    let mut i = 1;
    while i*i <= n { res.push((i, i, n/i)); i += 1; }
    // i = sqrt(n)+1
    for j in (1..=n/i).rev() {
        if n/j - n/(j+1) != 0 { res.push((n/(j+1)+1, n/j, j)); }
    }
    res
}

/// `(a, b, c)`: `for i in a..=b, ceil(n/i) == c`
pub fn quotient_ceil(n: usize) -> Vec<(usize, usize, usize)> {
    let mut res = vec![];
    let mut i = 1;
    while (i+1)*(i+1) <= n { res.push((i, i, (n+i-1)/i)); i += 1; }
    // i = sqrt(n)
    for j in (2..=(n+i-1)/i).rev() {
        let l = (n+j-1)/j;
        let r = (n-1)/(j-1);
        assert!((n+l-1)/l == j && (n+r-1)/r == j);
        res.push(((n+j-1)/j, (n-1)/(j-1), j));
    }
    res.push((n, n, 1));
    res
}
