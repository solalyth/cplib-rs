pub fn minplus(a: &[i64], b: &[i64]) -> Vec<i64> {
    let (mut res, mut i, mut j) = (vec![a[0]+b[0]], 1, 1);
    while i < a.len() && j < b.len() {
        if a[i]-a[i-1] < b[j]-b[j-1] {
            res.push(a[i]-a[i-1]);
            i += 1;
        } else {
            res.push(b[j]-b[j-1]);
            j += 1;
        }
    }
    for i in i..a.len() { res.push(a[i]-a[i-1]); }
    for j in j..b.len() { res.push(b[j]-b[j-1]); }
    for i in 1..res.len() { res[i] += res[i-1]; }
    res
}
