/// XOR linked tree もどき。
/// xor じゃなくて加減にすれば deg も要らなくねと思って書いた。
/// 隣接リスト作ったら旨味が減るが、作らないと木の汎用ライブラリにしづらくて、困った。

const A: usize = 32;
const B: usize = 1<<A;

#[allow(dead_code)]
pub struct Tree<Op: DataOp> {
    root: usize,
    par: Vec<usize>,
    dat: Vec<Op::T>,
    // / 根からの順序
    // order: Vec<u32>,
}

impl<Op: DataOp> Tree<Op> {
    pub fn new(n: usize) -> Self {
        Self { root: 0, par: vec![0; n], dat: vec![Op::T::default(); n] }
    }
    
    pub fn add(&mut self, u: usize, v: usize, w: Op::T) {
        self.par[u] += v+B; self.par[v] += u+B;
        Op::xor(w, &mut self.dat[u]); Op::xor(w, &mut self.dat[v]);
    }
    
    pub fn build(&mut self, root: usize) {
        let n = self.dat.len();
        self.par[root] = !0;
        
        for mut i in 0..n {
            while self.par[i]>>A == 1 {
                // self.order.push(i as u32);
                let p = self.par[i] & (B-1);
                self.par[p] -= i+B;
                Op::xor(self.dat[i], &mut self.dat[p]);
                i = p;
            }
        }
        // self.order.push(root as u32);
        
        for i in 0..n { self.par[i] &= B-1; }
        self.root = root;
        self.par[root] = !0;
    }
    
    pub fn par(&self, i: usize) -> usize { self.par[i] }
    // pub fn order(&self) -> impl DoubleEndedIterator<Item = usize> { self.order.iter().map(|&i| i as usize) }
}


pub trait DataOp {
    type T: Copy + Default;
    fn xor(x: Self::T, y: &mut Self::T);
}

pub struct Xor<T>;
impl<T: Copy + Default + std::ops::BitXorAssign> DataOp for Xor<T> {
    type T = T;
    fn xor(x: Self::T, y: &mut Self::T) {
        *y ^= x;
    }
}
impl DataOp for Xor<()> {
    type T = ();
    fn xor(_: Self::T, _: &mut Self::T) {}
}
