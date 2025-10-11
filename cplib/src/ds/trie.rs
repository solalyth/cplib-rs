use std::fmt::Debug;

pub struct Trie<const N: usize> {
    /// `dat[(N+1)*idx + 0..N]` は遷移先、`dat[(N+1)*idx + N]` は遷移元である。
    dat: Vec<usize>,
    len: usize
}

impl<const N: usize> Trie<N> {
    pub fn new() -> Self {
        Self { dat: vec![!0; (N+1)*1024], len: 1 }
    }
    
    pub fn len(&self) -> usize { self.len }
    
    fn new_cell(&mut self) -> usize {
        if self.dat.len() == self.len * (N+1) {
            let len = self.dat.len()/(N+1);
            self.dat.resize(len*(N+1)*2, !0);
            for i in len..len*2 { self.dat[i*(N+1)+N] = 0; }
        }
        self.len += 1;
        self.len - 1
    }
    
    pub fn parent(&self, idx: usize) -> Option<usize> {
        assert!(idx < self.len);
        if idx != 0 { Some(self.dat[idx*(N+1)+N]) } else { None }
    }
    
    /// 次の遷移先が存在するなら返す。
    pub fn check_next(&self, idx: usize, c: usize) -> Option<usize> {
        assert!(c < N);
        if self.dat[idx*(N+1)+c] != !0 { Some(self.dat[idx*(N+1)+c]) } else { None }
    }
    
    /// 次の遷移先を返す。存在しない場合はノードを追加する。
    pub fn next(&mut self, idx: usize, c: usize) -> usize {
        assert!(c < N);
        if self.dat[idx*(N+1)+c] == !0 {
            let nx = self.new_cell();
            self.dat[idx*(N+1)+c] = nx;
            self.dat[nx*(N+1)+N] = idx;
        }
        self.dat[idx*(N+1)+c]
    }
    
    pub fn insert(&mut self, s: impl Iterator<Item = usize>) -> Vec<usize> {
        let mut res = vec![0];
        let mut cur = 0;
        for c in s {
            cur = self.next(cur, c);
            res.push(cur);
        }
        res
    }
    
    /// Aho-Corasick 法における遷移先を計算する。`(ac, ac_nx)`
    /// 
    /// `ac[i]`: 先頭から pop していって、初めてマッチする遷移先を返す。
    /// `ac_nx[i*26+c]`: 文字 `c` を追加したときの遷移先を返す。
    /// 
    /// 遷移先とは、trie 上に存在する最長 suffix の index のこと。
    pub fn aho_corasick(&self) -> (Vec<usize>, Vec<usize>) {
        let mut ac = vec![0; self.len];
        let mut que = vec![0];
        while let Some(p) = que.pop() {
            // ac[p] が求まっているものとして、子ノードの ac[c] を求める
            for c in 0..N {
                let c = self.dat[p*(N+1)+c];
                if c == !0 { continue; }
                let mut cur = p;
                ac[c] = loop {
                    if cur == 0 { break 0; }
                    cur = ac[cur];
                    if self.dat[cur*(N+1)+c] != !0 { break self.dat[cur*(N+1)+c]; }
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
