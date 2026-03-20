//! [`Trie`], [`AhoCorasick`]

use std::fmt::Debug;

const MASK: usize = (1<<32)-1;

/// 文字種 `N` の Trie を管理する。
pub struct Trie<const N: usize> {
    /// `dat[(N+1)*idx + c=0..N]`: 遷移先
    /// `dat[(N+1)*idx + N]`: (遷移c)<<32 + 遷移元
    dat: Vec<usize>,
    /// 使用しているノード数
    len: usize
}

impl<const N: usize> Trie<N> {
    pub fn new() -> Self {
        Self { dat: vec![!0; (N+1)*1024], len: 1 }
    }
    
    pub fn clear(&mut self) {
        self.dat[..(N+1)*self.len].fill(!0);
        self.len = 1;
    }
    
    fn next_cell(&mut self) -> usize {
        if self.dat.len() == (N+1) * self.len {
            self.dat.resize(self.dat.len()*2, !0);
        }
        self.len += 1;
        self.len-1
    }
    
    /// `next(par, c) == idx` となる `(par, c)` を返す。
    /// 
    /// # Panics
    /// 
    /// - if not `idx != 0 && idx < trie.len`
    pub fn parent(&self, idx: usize) -> (usize, usize) {
        assert!(idx != 0 && idx < self.len);
        let t = self.dat[(N+1)*idx+N];
        (t & MASK, t >> 32)
    }
    
    /// 遷移先が存在するなら返す。
    /// 
    /// # Panics
    /// 
    /// - if not `idx < self.len && c < N`
    pub fn check_next(&self, idx: usize, c: usize) -> Option<usize> {
        assert!(idx < self.len && c < N);
        let t = self.dat[(N+1)*idx+c];
        if t != !0 { Some(t) } else { None }
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
            self.dat[(N+1)*nx+N] = (c<<32) + idx;
        }
        self.dat[(N+1)*idx+c]
    }
    
    /// 文字列 `iter` に対応するインデックス列を返す。存在しないならば新しくノードを作る。`res[i] = Node-idx of iter[..i]`
    pub fn insert(&mut self, iter: impl IntoIterator<Item = usize>) -> Vec<usize> {
        let mut res = vec![0];
        let mut cur = 0;
        for c in iter {
            cur = self.next(cur, c);
            res.push(cur);
        }
        res
    }
    
    pub fn aho_corasick<'a>(&'a self) -> AhoCorasick<'a, N> {
        let mut dat = vec![0; (N+1)*self.len];
        
        for i in 0..self.len {
            // next[..i], fail[..=i] が計算されている
            for c in 0..N {
                let (j, fj) = (self.dat[(N+1)*i+c], dat[(N+1)*dat[(N+1)*i+N]+c]);
                dat[(N+1)*i+c] = if j != !0 { dat[(N+1)*j+N] = fj; j } else { fj };
            }
        }
        
        AhoCorasick { trie: &self, dat }
    }
}

impl<const N: usize> Debug for Trie<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert!(N <= 26);
        let mut s = vec![String::from("(deleted)"); self.len];
        s[0].clear();
        for i in 0..self.len {
            if &s[i] == "(deleted)" { continue; }
            for c in 0..N {
                let j = self.dat[(N+1)*i+c];
                if j == !0 { continue; }
                s[j] = format!("{}{}", s[i], (b'a' + c as u8) as char);
            }
        }
        
        let mut res = String::new();
        for s in &s[1..] { res += ", "; res += s; }
        write!(f, "[(empty){res}]")
    }
}


pub struct AhoCorasick<'a, const N: usize> {
    trie: &'a Trie<N>,
    /// `dat[(N+1)*idx + c=0..N]`: 遷移先
    /// `dat[(N+1)*idx + N]`: 失敗遷移先, ただし fail(0) = 0
    dat: Vec<usize>
}

impl<const N: usize> AhoCorasick<'_, N> {
    /// `idx` に対応した文字列に文字 `c` を連結したものについて、(適切に fail を取るなどして) それに対応すべきノードを返す。
    pub fn next(&self, idx: usize, c: usize) -> usize {
        assert!(idx < self.trie.len && c < N);
        self.dat[(N+1)*idx+c]
    }
    /// `fail(0) = 0` に注意せよ。
    pub fn fail(&self, idx: usize) -> usize {
        assert!(idx < self.trie.len);
        self.dat[(N+1)*idx+N]
    }
}
