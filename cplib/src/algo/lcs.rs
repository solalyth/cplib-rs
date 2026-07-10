pub fn lcs<T: PartialEq>(s: &[T], t: &[T]) -> Vec<Vec<usize>> {
    let mut dp = vec![vec![0; t.len()+1]; s.len()+1];
    for i in 0..s.len() {
        for j in 0..t.len() {
            dp[i+1][j] = dp[i+1][j].max(dp[i][j]);
            dp[i][j+1] = dp[i][j+1].max(dp[i][j]);
            if s[i] == t[j] { dp[i+1][j+1] = dp[i+1][j+1].max(dp[i][j]+1); }
        }
    }
    dp
}

pub fn lcs_restore<T: PartialEq + Clone>(s: &[T], t: &[T], dp: &Vec<Vec<usize>>) -> Vec<T> {
    let (mut i, mut j, mut res) = (s.len(), t.len(), vec![]);
    while i|j != 0 {
        if i != 0 && dp[i][j] == dp[i-1][j] { i -= 1; }
        if j != 0 && dp[i][j] == dp[i][j-1] { j -= 1; }
        if i|j != 0 && dp[i][j] == dp[i-1][j-1]+1 { res.push(s[i-1].clone()); i -= 1; j -= 1; }
    }
    res.reverse();
    res
}
