use std::fmt::Debug;

/// 文字種 `N` の Trie
pub struct Trie<const N: usize> {
    /// `dat[(N+1)*idx + 0..N]` は遷移先、`dat[(N+1)*idx + N]` は遷移元である。存在しない場合は `!0` が入る。
    dat: Vec<usize>,
    /// 実際に使用しているノード数
    len: usize
}

impl<const N: usize> Trie<N> {
    pub fn new() -> Self {
        Self { dat: vec![!0; (N+1)*1024], len: 1 }
    }
    
    pub fn len(&self) -> usize { self.len }
    
    pub fn clear(&mut self) {
        for i in 0..(N+1)*self.len {
            self.dat[i] = !0;
        }
        self.len = 1;
    }
    
    fn next_cell(&mut self) -> usize {
        if self.dat.len() == (N+1) * self.len {
            self.dat.resize(self.dat.len()*2, !0);
        }
        self.len += 1;
        self.len-1
    }
    
    /// 親ノードを返す。
    pub fn parent(&self, idx: usize) -> Option<usize> {
        assert!(idx < self.len);
        if idx != 0 { Some(self.dat[(N+1)*idx+N]) } else { None }
    }
    
    /// 遷移先が存在するなら返す。
    /// 
    /// # Panics
    /// 
    /// - if not `idx < self.len && c < N`
    pub fn check_next(&self, idx: usize, c: usize) -> Option<usize> {
        assert!(idx < self.len && c < N);
        if self.dat[(N+1)*idx+c] != !0 { Some(self.dat[(N+1)*idx+c]) } else { None }
    }
    
    /// 遷移先を返す。存在しない場合はノードを追加する。
    /// 
    /// # Panics
    /// 
    /// - if not `idx < self.len && c < N`
    pub fn next(&mut self, idx: usize, c: usize) -> usize {
        assert!(idx < self.len && c < N);
        if self.dat[(N+1)*idx+c] == !0 {
            let nx = self.next_cell();
            self.dat[(N+1)*idx+c] = nx;
            self.dat[(N+1)*nx+N] = idx;
        }
        self.dat[(N+1)*idx+c]
    }
    
    /// 文字列 `iter` を挿入する。`res[i] = Node of iter[..i]`
    pub fn insert(&mut self, iter: impl IntoIterator<Item = usize>) -> Vec<usize> {
        let mut res = vec![0];
        let mut cur = 0;
        for c in iter {
            cur = self.next(cur, c);
            res.push(cur);
        }
        res
    }

    /// Aho-Corasick 法における遷移先を計算する。`res = (ac, ac_nx)`
    /// 
    /// `ac[i]`: 先頭から削っていって初めてマッチする Node を返す。ただし `ac[0] = !0`
    /// `ac_nx[N*i+c]`: ノード `i` が表す文字列に文字 `c` を追加し、(必要ならば先頭から削っていって) 初めてマッチする Node を返す。
    pub fn aho_corasick(&self) -> (Vec<usize>, Vec<usize>) {
        let mut ac = vec![!0; self.len];
        let mut que = vec![0];
        while let Some(p) = que.pop() {
            // 帰納条件: ac[p] が求まっている
            for c in 0..N {
                let c = self.dat[p*(N+1)+c];
                if c == !0 { continue; }
                let mut cur = ac[p];
                ac[c] = loop {
                    if cur == !0 { break 0; }
                    if self.dat[cur*(N+1)+c] != !0 { break self.dat[cur*(N+1)+c]; }
                    cur = ac[cur];
                };
                que.push(c);
            }
        }
        
        let mut ac_nx = vec![0; self.len*N];
        for i in 0..ac.len() {
            for c in 0..N {
                let mut cur = i;
                ac_nx[i*N+c] = loop {
                    if self.dat[cur*(N+1)+c] != !0 { break self.dat[cur*(N+1)+c]; }
                    if cur == 0 { break cur; }
                    cur = ac[cur];
                };
            }
        }
        
        (ac, ac_nx)
    }
}


impl<const N: usize> Debug for Trie<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tmp = vec![None; self.len];
        tmp[0] = Some(String::new());
        let mut stk = vec![0];
        while let Some(i) = stk.pop() {
            for c in 0..N {
                let nx = self.dat[i*(N+1)+c];
                if nx == !0 { continue; }
                let mut s = tmp[i].clone().unwrap();
                s.push((b'a' + c as u8) as char);
                tmp[nx] = Some(s);
                stk.push(nx);
            }
        }
        
        let mut res = String::from("");
        for s in &tmp[1..] {
            res += ", ";
            if let Some(s) = s {
                res += s;
            } else {
                res += "*";
            }
        }
        write!(f, "[@{res}]")
    }
}
