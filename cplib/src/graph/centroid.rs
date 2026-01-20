pub fn centroids(edge: &Vec<Vec<usize>>, dead: &Vec<bool>) -> Vec<usize> {
    let n = edge.len();
    let (mut dp, mut stk, mut par, mut res) = (vec![1; n], vec![], vec![!0; n], vec![]);
    for i in 0..n {
        if dead[i] || par[i] != !0 { continue; }
        stk.push(!i);
        stk.push(i);
        while let Some(u) = stk.pop() {
            if u < !u {
                for &v in &edge[u] {
                    if dead[v] || v == par[u] { continue; }
                    par[v] = u;
                    stk.push(!v);
                    stk.push(v);
                }
            } else {
                if par[!u] != !0 { dp[par[!u]] += dp[!u]; }
            }
        }
        let mut cur = i;
        while let Some(&u) = edge[cur].iter().find(|&&u| !dead[u] && u != par[cur] && dp[i] < dp[u]*2) { cur = u; }
        res.push(cur);
    }
    
    res
}
