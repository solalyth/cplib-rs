/// 狭義単調増加
pub fn lis_lt(rank: &[usize]) -> (usize, Vec<usize>) {
    let n = rank.len();
    let (mut lis, mut idx) = (vec![], vec![0; rank.len()]);
    for i in 0..n {
        let x = rank[i];
        let j = lis.partition_point(|&y| y < x);
        if j == lis.len() { lis.push(x); } else { lis[j] = x; }
        idx[i] = j;
    }
    lis.fill(0);
    let l = lis.len();
    for i in (0..rank.len()).rev() {
        let j = idx[i];
        if j != l-1 && rank[i] >= lis[j+1] { idx[i] = !0; } else { lis[j] = rank[i]; }
    }
    
    (l, idx)
}
