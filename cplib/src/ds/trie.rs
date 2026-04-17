//! [`Trie`], [`AhoCorasick`]

use std::fmt::Debug;

const MASK: usize = (1<<32)-1;

/// 文字 `0..W` からなる文字列の Trie を管理する。
pub struct Trie<const W: usize> {
    /// `dat[(W+1)*idx + c=0..W]`: 遷移先
    /// `dat[(W+1)*idx + W]`: (遷移c)<<32 + 遷移元
    dat: Vec<usize>,
}

impl<const W: usize> Trie<W> {
    pub fn new() -> Self {
        Self { dat: vec![!0; W+1] }
    }
    
    pub fn len(&self) -> usize { self.dat.len() / (W+1) }
    
    pub fn clear(&mut self) { self.dat.clear(); }
    
    /// `next(par, c) == idx` となる `(par, c)` を返す。
    /// 
    /// # Panics
    /// 
    /// - if not `idx != 0 && idx < trie.len`
    pub fn parent(&self, idx: usize) -> (usize, usize) {
        assert!(idx != 0 && idx < self.len());
        let t = self.dat[(W+1)*idx+W];
        (t & MASK, t >> 32)
    }
    
    /// 遷移先が存在するなら返す。存在しない場合は `!0` を返す。
    /// 
    /// # Panics
    /// 
    /// - if not `idx < self.len && c < W`
    pub fn check_next(&self, idx: usize, c: usize) -> usize {
        assert!(idx < self.len() && c < W);
        self.dat[(W+1)*idx+c]
    }
    
    /// 遷移先を返す。存在しない場合はノードを追加する。
    /// 
    /// # Panics
    /// 
    /// - if not `idx < self.len && c < W`
    pub fn next(&mut self, idx: usize, c: usize) -> usize {
        assert!(idx < self.len() && c < W);
        if self.dat[(W+1)*idx+c] == !0 {
            let nx = self.len();
            for _ in 0..W+1 { self.dat.push(!0); }
            self.dat[(W+1)*idx+c] = nx;
            self.dat[(W+1)*nx+W] = (c<<32) + idx;
        }
        self.dat[(W+1)*idx+c]
    }
    
    /// 文字列 `iter` に対応するインデックス列を返す。存在しないならば新しくノードを作る。`res[i] = node-idx of iter[..i]`
    pub fn insert(&mut self, iter: impl IntoIterator<Item = usize>) -> Vec<usize> {
        let mut res = vec![0];
        let mut cur = 0;
        for c in iter {
            cur = self.next(cur, c);
            res.push(cur);
        }
        res
    }
    
    /// [`AhoCorasick`] を構築する。
    pub fn aho_corasick(&self) -> AhoCorasick<W> {
        let mut dat = vec![0; (W+1)*self.len()];
        
        for i in 0..self.len() {
            // next[..i], fail[..=i] が計算されている
            for c in 0..W {
                let (j, fj) = (self.dat[(W+1)*i+c], dat[(W+1)*dat[(W+1)*i+W]+c]);
                if j != !0 {
                    dat[(W+1)*j+W] = fj;
                    dat[(W+1)*i+c] = j;
                } else {
                    dat[(W+1)*i+c] = fj;
                }
            }
        }
        
        AhoCorasick { trie_len: self.len(), dat }
    }
}

impl<const W: usize> Debug for Trie<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        assert!(W <= 26);
        let mut s = vec![String::from("(deleted)"); self.len()];
        s[0].clear();
        for i in 0..self.len() {
            if &s[i] == "(deleted)" { continue; }
            for c in 0..W {
                let j = self.dat[(W+1)*i+c];
                if j == !0 { continue; }
                s[j] = format!("{}{}", s[i], (b'a' + c as u8) as char);
            }
        }
        
        let mut res = String::new();
        for s in &s[1..] { res += ", "; res += s; }
        write!(f, "[(empty){res}]")
    }
}



/// [`Trie`] から構築される Aho-Corasick オートマトン。
/// 
/// パターンの検索の実装は面倒になったので dictionary link をサボっている。個数だけなら、fail する直前のノードに対し trie 木上の累積和を見ることで求まる。
pub struct AhoCorasick<const W: usize> {
    trie_len: usize,
    /// `dat[(W+1)*idx + c=0..W]`: 遷移先
    /// `dat[(W+1)*idx + W]`: 失敗遷移先, ただし fail(0) = 0
    dat: Vec<usize>,
}

impl<const W: usize> AhoCorasick<W> {
    /// 文字列 `trie[idx] + c` に対応すべき node の index を返す。
    pub fn next(&self, idx: usize, c: usize) -> usize {
        assert!(idx < self.trie_len && c < W);
        self.dat[(W+1)*idx+c]
    }
    /// `fail(0) = 0` に注意。
    pub fn fail(&self, idx: usize) -> usize {
        assert!(idx < self.trie_len);
        self.dat[(W+1)*idx+W]
    }
}
