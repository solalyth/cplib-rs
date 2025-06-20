//! 一般重み付きグラフにおける、全頂点ペアの最小コスト経路問題 `O(N³)`
//! 
//! 負辺が存在せず `M << N²` であるときは、dijkstra N 回 `O(NM + N²logN)` のほうが早い。
//! 
//! # Algorithm
//! 
//! 1. `cost[i][j]` = 「`i -> j` の最小コスト」と定義し、`d[i][i] = 0, d[u][v] = w, others = INF` で初期化。
//! 2. `j in 0..N` についてループする。
//!   + 帰納条件: 経由頂点が全て `0..j` に限定された状況での `cost` が求まっている。  
//!   + 更新: `j` を経由した最小コスト経路で chmin する。`for (i, k) { chmin!(d[i][k]; d[i][j] + d[j][k]); }` とすればよい。
//! 3. `cost[j][j] < 0 && cost[i][j], cost[j][k] != MAX` ならば `cost[i][k] = MIN` にする。

use crate::{chmin, nest};

#[allow(non_camel_case_types)]
type int = i64;

pub struct WarshallFloyd {
    /// `MIN` = 負閉路によりコストを `-INF` にできる。`MAX` = 到達不能。
    pub cost: Vec<Vec<int>>,
    /// `next[i][j]` = `i -> j` 最小コスト経路の、`i` の一つ次の頂点
    pub next: Vec<Vec<usize>>
}

impl WarshallFloyd {
    pub fn new(len: usize, edge: &[(usize, usize, int)]) -> Self {
        let mut cost = nest![int::MAX; len; len];
        let mut next: Vec<Vec<usize>> = vec![(0..len).collect(); len];
        for i in 0..len { cost[i][i] = 0; }
        for &(u, v, w) in edge { chmin!(cost[u][v]; w); }
        for j in 0..len {
            for i in 0..len {
                for k in 0..len {
                    if cost[i][j] == int::MAX || cost[j][k] == int::MAX { continue; }
                    if chmin!(cost[i][k]; cost[i][j].wrapping_add(cost[j][k])) { next[i][k] = next[i][j]; }
                }
            }
        }
        for j in 0..len {
            if !(cost[j][j] < 0) { continue; }
            for i in 0..len {
                for k in 0..len { if cost[i][j] != int::MAX && cost[j][k] != int::MAX { cost[i][k] = int::MIN; } }
            }
        }
        WarshallFloyd { cost, next }
    }
    
    /// `u -> v` への最小コスト経路 `[u, ..., v]` を返す。
    /// 
    /// # Panics
    /// 
    /// + if `cost[u][v] == MIN, MAX`
    pub fn route(&self, mut u: usize, v: usize) -> Vec<usize> {
        assert!(self.cost[u][v] != int::MIN && self.cost[u][v] != int::MAX);
        let mut ret = vec![u];
        while u != v { u = self.next[u][v]; ret.push(u); }
        ret
    }
}
