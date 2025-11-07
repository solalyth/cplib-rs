//! [`Trie`], [`AhoCorasick`]

use std::fmt::Debug;

/// 文字種 `N` の Trie を持つ構造体。削除はできない。
/// 
/// # Memo
/// 
/// 辺の削除・付け替えがしたいケースはある？良い感じに操作しないとループができて嫌そう。
pub struct Trie<const N: usize> {
    /// `dat[(N+1)*idx + c=0..N]`: 遷移先
    /// `dat[(N+1)*idx + N]`: 遷移元
    dat: Vec<usize>,
    /// 使用しているノード数
    len: usize
}

impl<const N: usize> Trie<N> {
    pub fn new() -> Self {
        Self { dat: vec![!0; (N+1)*1024], len: 1 }
    }
    
    /// 使用しているノード数を返す。
    pub fn len(&self) -> usize { self.len }
    
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
    
    /// 親ノードを返す。
    pub fn parent(&self, idx: usize) -> Option<usize> {
        assert!(idx < self.len);
        if idx == 0 { None } else { Some(self.dat[(N+1)*idx+N]) }
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
    
    /// 文字列 `iter` を挿入する。`res[i] = Node-idx of iter[..i]`
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
                let (j, fc) = (self.dat[(N+1)*i+c], dat[(N+1)*dat[(N+1)*i+N]+c]);
                dat[(N+1)*i+c] = if j != !0 { dat[(N+1)*j+N] = fc; j } else { fc };
            }
        }
        
        AhoCorasick { trie: &self, dat }
    }
}

impl<const N: usize> Debug for Trie<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert!(N <= 26);
        let mut s = vec![String::new(); self.len];
        for i in 0..self.len {
            for c in 0..N {
                let j = self.dat[(N+1)*i+c];
                if j == !0 { continue; }
                s[j] = format!("{}{}", s[i], (b'a' + c as u8) as char);
            }
        }
        
        let mut res = String::new();
        for s in &s[1..] { res += ", "; res += s; }
        write!(f, "[*{res}]")
    }
}


pub struct AhoCorasick<'a, const N: usize> {
    trie: &'a Trie<N>,
    /// `dat[(N+1)*idx + c=0..N]`: 遷移先
    /// `dat[(N+1)*idx + N]`: fail 先, ただし fail(0) = 0
    dat: Vec<usize>
}

impl<const N: usize> AhoCorasick<'_, N> {
    pub fn next(&self, idx: usize, c: usize) -> usize {
        assert!(idx < self.trie.len && c < N);
        self.dat[(N+1)*idx+c]
    }
    pub fn fail(&self, idx: usize) -> Option<usize> {
        assert!(idx < self.trie.len);
        if idx == 0 { None } else { Some(self.dat[(N+1)*idx+N]) }
    }
}
