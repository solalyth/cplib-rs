use crate::cplib::ds::csr::Edge;

pub fn centroids(edge: &Edge, dead: &Vec<bool>) -> Vec<usize> {
    let n = edge.idx_len();
    let (mut sz, mut stk, mut par, mut res) = (vec![1; n], vec![], vec![!0; n], vec![]);
    for i in 0..n {
        if dead[i] || par[i] != !0 { continue; }
        stk.extend([!i, i]);
        while let Some(u) = stk.pop() {
            if u < !u {
                for &v in &edge[u] {
                    if dead[v] || v == par[u] { continue; }
                    par[v] = u;
                    stk.extend([!v, v]);
                }
            } else {
                if par[!u] != !0 { sz[par[!u]] += sz[!u]; }
            }
        }
        let mut cur = i;
        while let Some(&u) = edge[cur].iter().find(|&&u| !dead[u] && u != par[cur] && sz[i] < sz[u]*2) { cur = u; }
        res.push(cur);
    }
    
    res
}
