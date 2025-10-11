//! Longest Common Subsequence (最長共通部分列)
//! 
//! `dp[i][j]` = `lcs(s[..i], t[..j])` と置くと `dp[i][j] -> dp[i+1][j], dp[i][j+1], dp[i+1][j+1]` が計算できる。



pub fn lcs<T: PartialEq + Clone>(s: &[T], t: &[T]) -> Vec<T> {
    let mut dp = vec![vec![0; t.len()+1]; s.len()+1];
    for i in 0..s.len() {
        for j in 0..t.len() {
            if s[i] == t[j] {
                dp[i+1][j+1] = dp[i][j]+1;
            } else {
                dp[i+1][j+1] = dp[i+1][j].max(dp[i][j+1]);
            }
        }
    }
    let mut ans = vec![];
    let mut i = s.len();
    let mut j = t.len();
    while i != 0 && j != 0 {
        if dp[i][j] == dp[i-1][j-1]+1 {
            ans.push(s[i-1].clone());
            i -= 1;
            j -= 1;
        } else if dp[i][j-1] <= dp[i-1][j] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    ans.reverse();
    ans
}
