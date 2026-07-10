pub struct BitVec {
    bit: Vec<u64>,
    sum: Vec<u32>,
    c0: usize
}

impl BitVec {
    fn new(v: &mut [usize], b: u32, buf: &mut [Vec<usize>; 2]) -> Self {
        let mut bit = vec![];
        let mut sum = vec![0];
        
        buf[0].clear(); buf[1].clear();
        
        let mut s = 0;
        for w in v.chunks(64) {
            let mut bs = 0;
            for (i, &x) in w.iter().enumerate() {
                if x>>b&1 == 1 { bs ^= 1<<i; s += 1; }
                buf[x>>b&1].push(x);
            }
            bit.push(bs);
            sum.push(s);
        }
        
        bit.push(0);
        
        v[..buf[0].len()].copy_from_slice(&buf[0]);
        v[buf[0].len()..].copy_from_slice(&buf[1]);
        
        Self { bit, sum, c0: v.len() - s as usize }
    }
    
    pub fn rank(&self, i: usize) -> [usize; 2] {
        let (p, q) = (i/64, i%64);
        let c1 = (self.sum[p] + (self.bit[p]&((1<<q)-1)).count_ones()) as usize;
        [i-c1, c1]
    }
    
    pub fn calc(&self, l: usize, r: usize) -> [usize; 4] {
        let ([l0, l1], [r0, r1]) = (self.rank(l), self.rank(r));
        [l0, r0, self.c0+l1, self.c0+r1]
    }
}



/// Wavelet Matrix
/// 
/// 区間 `[l, r)` 内の binary trie を仮想的に扱うことができるデータ構造だと思うと分かりやすい。
/// 
/// クエリの処理中は区間を変更できないため、自由度が小さいほうを区間にするとよい。
pub struct WaveletMatrix {
    /// dat[i]: layer[i+1] から layer[i] への遷移で i bit 目を見る
    pub dat: Vec<BitVec>
}

impl WaveletMatrix {
    pub fn new_auto(v: Vec<usize>) -> Self {
        let b = v.iter().max().unwrap_or(&0).max(&1).ilog2()+1; // ceil(log2)
        Self::new(v, b)
    }
    
    pub fn new(mut v: Vec<usize>, b: u32) -> Self {
        let mut buf = [vec![], vec![]];
        let mut dat = vec![];
        
        for b in (0..b).rev() {
            dat.push(BitVec::new(&mut v, b, &mut buf));
        }
        dat.reverse();
        Self { dat }
    }
    
    pub fn b(&self) -> usize { self.dat.len() }
    pub fn xsup(&self) -> usize { 1<<self.b() }
    
    pub fn get_pos(&self, mut i: usize) -> Vec<usize> {
        let b = self.b();
        let mut res = vec![0; b+1]; res[b] = i;
        for b in (0..b).rev() {
            let l0 = self.dat[b].rank(i)[0];
            if i>>b&1 == 0 { i = l0; } else { i = self.dat[b].c0+i-l0; }
            res[b] = i;
        }
        res
    }
    
    /// インデックス `[l, r)` 内にあるキーのうち `k` 番目の値を返す。
    pub fn simple_kth(&self, mut l: usize, mut r: usize, mut k: usize) -> usize {
        assert!(k < r-l);
        let mut x = 0;
        for b in (0..self.b()).rev() {
            let [l0, r0, l1, r1] = self.dat[b].calc(l, r);
            (l, r) = if k < r0-l0 { (l0, r0) } else { k -= r0-l0; x ^= 1<<b; (l1, r1) };
        }
        x
    }
    
    /// 
    pub fn simple_cnt(&self, mut l: usize, mut r: usize, x: usize) -> usize {
        for b in (0..self.b()).rev() {
            let [l0, r0, l1, r1] = self.dat[b].calc(l, r);
            (l, r) = if x>>b&1 == 0 { (l0, r0) } else { (l1, r1) };
        }
        r-l
    }
    
    /// インデックス `[l, r)` 内でキーが `[xl, xr)` である要素を区間に分割する。返り値は `(layer i, L, R)` の列。
    pub fn decomp(&self, l: usize, r: usize, xl: usize, mut xr: usize) -> Vec<(usize, usize, usize)> {
        xr = xr.min(self.xsup());
        if l >= r || xl >= xr { return vec![]; }
        let (mut stk, mut res) = (vec![(self.b(), l, r, xl, xr)], vec![]);
        while let Some((b, l, r, xl, xr)) = stk.pop() {
            if xr-xl == 1<<b { res.push((b, l, r)); continue; }
            let [l0, r0, l1, r1] = self.dat[b-1].calc(l, r);
            let xm = (xl>>b-1 | 1)<<b-1;
            if xl < xm { stk.push((b-1, l0, r0, xl, xr.min(xm))); }
            if xm < xr { stk.push((b-1, l1, r1, xl.max(xm), xr)); }
        }
        res
    }
}
