/// `j in inv_csr[inv_idx[i]..inv_idx[i+1]]` => `f[j] == i`.<br>
/// `j in root_csr[root_idx[i]..root_idx[i+1]]` => `id[j] == i`.
pub struct FunctionalGraph {
    #[allow(dead_code)]
    f: Vec<usize>,
    
    inv_csr: Vec<usize>,
    inv_idx: Vec<usize>,
    
    id: Vec<usize>,
    root_csr: Vec<usize>,
    /// id -> range of root_csr
    root_idx: Vec<usize>,
    root_inv: Vec<usize>,
    order: Vec<usize>
}

impl FunctionalGraph {
    pub fn new(f: impl IntoIterator<Item = usize>) -> Self {
        let f = f.into_iter().collect::<Vec<_>>();
        let (mut inv_csr, mut inv_idx, mut id, mut root_csr, mut root_idx, mut root_inv, mut order, mut cur) = (vec![0; f.len()], vec![0; f.len()+1], vec![0; f.len()], vec![], vec![0], vec![0; f.len()], vec![], 0);
        for i in 0..f.len() { inv_idx[f[i]+1] += 1; }
        let mut d = inv_idx[1..].to_vec();
        for i in 0..f.len() { if d[i] == 0 { order.push(i); } }
        for i in 0.. {
            let Some(&i) = order.get(i) else { break; };
            d[f[i]] -= 1;
            if d[f[i]] == 0 { order.push(f[i]); }
        }
        for s in 0..f.len() {
            if d[s] == 0 { continue; }
            let mut x = s;
            loop {
                d[x] = 0;
                inv_idx[x+1] -= 1;
                id[x] = cur;
                root_inv[x] = root_csr.len();
                root_csr.push(x);
                
                x = f[x];
                if x == s { break; }
            }
            root_idx.push(root_csr.len());
            cur += 1;
        }
        for i in 0..f.len() { inv_idx[i+1] += inv_idx[i]; }
        for &i in order.iter().rev() {
            inv_csr[inv_idx[f[i]]+d[f[i]]] = i;
            d[f[i]] += 1;
            id[i] = id[f[i]];
            root_inv[i] = root_inv[f[i]];
        }
        
        Self { f, inv_csr, inv_idx, id, root_csr, root_idx, root_inv, order }
    }
    
    pub fn id(&self, i: usize) -> usize { self.id[i] }
    pub fn id_len(&self) -> usize { self.root_idx.len()-1 }
    pub fn root(&self, i: usize) -> usize { self.root_csr[self.root_inv[i]] }
    pub fn roots(&self, id: usize) -> &[usize] { &self.root_csr[self.root_idx[id]..self.root_idx[id+1]] }
    pub fn inv_edges(&self, i: usize) -> &[usize] { &self.inv_csr[self.inv_idx[i]..self.inv_idx[i+1]] }
    pub fn topo_order(&self) -> &[usize] { &self.order }
}
