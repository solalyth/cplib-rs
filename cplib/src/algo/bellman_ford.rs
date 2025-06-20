//! 符号付き重み付きグラフでの、一始点からの最小コスト経路問題 `O(EV)`
//! 
//! # Algorithm
//! 
//! 1. 最大コスト(到達不能)で初期化し、辺ごとにコスト更新することを `N` 回行う。
//! 2. もう一度、辺ごとにコスト更新することを `N` 回行うが、更新できた場合は行き先を最小コストにする。
//! 
//! # 牛ゲー
//! 
//! 任意の `gl` に対して `max B[gl] = A[gl]-A[st]; subject to A[j]-A[i] <= f(i, j)` が解ける。
//! 
//! 条件 `A[j]-A[i] = d` は `A[j]-A[i] <= d && A[i]-A[j] <= -d` とすればよい。
//! 
//! ## 解法
//! 
//! `i -> j` に `cost = f(i, j)` の辺を貼ったグラフでの、`st -> gl` の最小コストが `B[gl]` と一致する。これは Bellman-Ford 法で解ける。
//! 
//! `B[gl] == MIN` であるとき、条件を満たす数列 `B` は存在しない。
//! 
//! ## 帰着可能問題
//! 
//! 符号を反転させれば、任意の `st` について `min A[gl]-A[st]; subject to f(i, j) <= A[j]-A[i]` が解ける。

use crate::chmin;

#[allow(non_camel_case_types)]
type int = i64;

pub struct BellmanFord {
    pub start: usize,
    /// `MIN` = 負閉路が存在し、コストを `-INF` に出来る。`MAX` = 到達不能。
    pub cost: Vec<int>,
    pub prev: Vec<usize>
}

impl BellmanFord {
    /// 有向重み付きグラフの、頂点 `start` からある頂点への最小コストを求める。`O(EV)`
    /// 
    /// # Input
    /// 
    /// + `cost: &[(u, v, w)]`: `u -> v` を通ると `+w` のコストが掛かることを表す。
    pub fn new(start: usize, len: usize, edge: &[(usize, usize, int)]) -> Self {
        let mut cost = vec![int::MAX; len];
        let mut prev = vec![usize::MAX; len];
        cost[start] = 0;
        
        for i in 1..=2*len {
            for &(u, v, w) in edge {
                if cost[u] == int::MAX { continue; }
                if cost[u] == int::MIN { prev[v] = u; cost[v] = int::MIN; continue; }
                if chmin!(cost[v]; cost[u]+w) {
                    prev[v] = u;
                    if len <= i { cost[v] = int::MIN; }
                }
            }
        }
        
        BellmanFord { start, cost, prev }
    }
    
    /// `start -> u` への経路 `[start, ..., u]` を返す。
    /// 
    /// # Panics
    /// 
    /// + `cost[u]` が `MIN, MAX` であるとき。
    pub fn route(&self, mut u: usize) -> Vec<usize> {
        assert!(self.cost[u] != int::MIN && self.cost[u] != int::MAX);
        let mut res = vec![u];
        while u != self.start { u = self.prev[u]; res.push(u); }
        res.reverse(); res
    }
}
