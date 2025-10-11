use std::{cell::Cell, /* fmt::Debug, */ mem::replace, ops::{Deref, DerefMut}, ptr::NonNull};
// use crate::cplib::util::func::to_bounds;


/// [`SplayTree`] に載せる演算用 trait
pub trait SplayOp {
    type Value: Clone;
    type Acc: Clone;
    type Lazy: Clone;
    
    /// `Value` を `Acc` 化する。
    fn to_acc(value: &Self::Value) -> Self::Acc;
    /// `Acc` の積を計算する。
    fn prod_acc(lhs: &Self::Acc, rhs: &Self::Acc) -> Self::Acc;
    /// `Lazy` を `Value` に反映させる。
    fn act_value(value: &mut Self::Value, lazy: &Self::Lazy);
    /// `Lazy` を `Acc` に反映させる。
    fn act_acc(acc: &mut Self::Acc, lazy: &Self::Lazy);
    /// `Lazy` の合成を行う。
    fn comp_lazy(lazy: &mut Self::Lazy, ad: &Self::Lazy);
}


/// 
pub struct SplayTree<Op: SplayOp>(Cell<Option<NodeRef<Op>>>);

impl<Op: SplayOp> SplayTree<Op> {
    pub fn new() -> Self { Self(Cell::new(None)) }
    pub fn is_empty(&self) -> bool { self.0.get().is_none() }
    pub fn len(&self) -> usize { self.0.get().map_or(0, |p| p.len()) }
    
    fn splay(&self, p: NodeRef<Op>) -> NodeRef<Op> {
        p.splay();
        self.0.set(Some(p));
        p
    }
    
    pub fn get(&self, idx: usize) -> Option<&Op::Value> {
        self.nth(idx);
        self.0.get().map(|p| unsafe { &p.0.as_ref().value })
    }
    
    /// `idx` 番目を根にして、`f(&mut root)`
    pub fn set<T>(&self, idx: usize, f: impl FnOnce(&mut Op::Value) -> T) -> Option<T> {
        let Some(mut r) = self.nth(idx) else { return None; };
        let res = f(&mut r.value);
        r.recalc();
        Some(res)
    }
    
    /// `idx` 番目のノードを根にして、返す。
    /// 
    /// # Panics
    /// 
    /// if not `idx < self.len()`. 特に空木について呼ぶことができない。
    fn nth(&self, mut idx: usize) -> Option<NodeRef<Op>> {
        if !(idx < self.len()) { return None; }
        let mut p = self.0.get().unwrap();
        loop {
            use std::cmp::Ordering::*;
            p.push();
            match idx.cmp(&p.cnt[0]) {
                Less => { p = p.child[0].unwrap(); }
                Equal => { self.splay(p); return Some(p); }
                Greater => { idx -= p.cnt[0]+1; p = p.child[1].unwrap(); }
            }
        }
    }
    
    fn first(&self) -> NodeRef<Op> {
        let mut p = self.0.get().unwrap();
        while let Some(c) = p.child[0] { p = c; }
        self.splay(p)
    }
    
    pub fn insert(&mut self, idx: usize, value: Op::Value) {
        assert!(idx <= self.len());
        let node = NodeRef::new(value);
        let Some(mut p) = self.0.get() else { self.0.set(Some(node)); return; };
        let mut cur = 0;
        loop {
            p.push();
            let pos = cur+p.cnt[0] < idx;
            if pos { cur += p.cnt[0]+1; }
            let Some(c) = p.child[pos as usize] else {
                connect(Some(p), Some(node), pos);
                self.splay(node);
                return;
            };
            p = c;
        }
    }
    
    pub fn partition_point(&self, pred: impl Fn(&Op::Value) -> bool) -> usize {
        let Some(mut p) = self.0.get() else { return 0; };
        let mut res = 0;
        loop {
            p.push();
            let pos = pred(&p.value);
            if pos { res += p.cnt[0]+1; }
            let Some(c) = p.child[pos as usize] else { return res; };
            p = c;
        }
    }
    
    
    
    pub fn split(&mut self, at: usize) -> SplayTree<Op> {
        assert!(at <= self.len());
        if at == self.len() { return SplayTree::new(); }
        if at == 0 { return replace(self, SplayTree::new()); }
        self.nth(at);
        let mut r = self.0.get().unwrap();
        let mut l = r.child[0].take().unwrap();
        
        l.push();
        l.parent = None;
        self.0.set(Some(l));
        
        r.recalc();
        SplayTree(Cell::new(Some(r)))
    }
    
    pub fn concat(&mut self, r: Self) {
        if r.is_empty() { return; }
        let mut r = r.first();
        r.child[0] = self.0.get();
        r.recalc();
        self.0.set(Some(r));
    }
    
    pub fn remove(&mut self, idx: usize) -> Op::Value {
        assert!(idx < self.len());
        let r = self.split(idx);
        let mp = if let Some(mut r) = r.nth(1) {
            let m = replace(&mut r.child[0], self.0.get()).unwrap();
            r.recalc();
            self.0.set(Some(r));
            m
        } else {
            r.0.get().unwrap()
        };
        unsafe { Box::from_raw(mp.0.as_ptr()).value }
    }
}



// impl<Op: SplayOp> Debug for SplayTree<Op> where Op::Value: Debug {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let Some(r) = self.0.get() else { write!(f, "<empty>") };
//         let mut stk = vec![];
        
//     }
// }

/*

/// Splay 木
/// 
/// # Features
/// 
/// - functions
///   + `new, is_empty, len`
///   + インデックス挿入・削除・取得 `insert, remove?, get(get_mut)`
///   + 分割・マージ `split_off, merge`
///   + 区間集約・遅延作用・反転 `fold, act, reverse`
///   + `iter, range, partition_point`
/// - traits
///   + `Clone`
///   + `Debug where Ops::Value: Debug`
///   + `FromIterator<Value>`
///   + `IntoIterator for &SplayTree`
///   + `Index<usize>`: `splay.get(i).unwrap()` と等価。
/// 
/// # Memo
/// 
/// - `splay` しかしないような関数は `&self` で、そうでない破壊的関数は `&mut self` にしてある。
pub struct SplayTree<Ops: SplayOp> (Cell<*mut Node<Ops>>);

impl<Ops: SplayOp> SplayTree<Ops> {
    fn root(&self) -> *mut Node<Ops> { self.0.get() }
    fn set_root(&self, ptr: *mut Node<Ops>) { self.0.set(ptr); }
    
    pub fn new() -> Self { Self(Cell::new(null_mut())) }
    pub fn is_empty(&self) -> bool { self.root().is_null() }
    pub fn len(&self) -> usize { Node::len(self.root()) }
    
    /// 値を挿入する。一番右なら `at = self.len` とすればよい。
    /// 
    /// # Panics
    /// 
    /// `not (at <= self.len)`
    pub fn insert(&mut self, at: usize, value: Ops::Value) {
        assert!(at <= self.len());
        let [l, r] = Node::split_at(self.root(), at);
        let node = Node::new(value);
        self.set_root(Node::merge_3(l, node, r));
    }
    
    /// 値を削除して返す。
    /// 
    /// # Panics
    /// 
    /// `not (at < self.len)`
    pub fn remove(&mut self, at: usize) -> Ops::Value {
        assert!(at < self.len());
        let (l, c, r) = Node::split_at_3(self.root(), at..=at).unwrap();
        self.set_root(Node::merge(l, r));
        unsafe { Box::from_raw(c) }.value
    }
    
    /// `tree[range]` 部分を切り離す。
    pub fn split_off(&mut self, range: impl RangeBounds<usize>) -> SplayTree<Ops> {
        let Some((l, c, r)) = Node::split_at_3(self.root(), range) else { return SplayTree::new() };
        self.set_root(Node::merge(l, r));
        Self(Cell::new(c))
    }
    
    /// `tree[range]` を逆順にする。
    pub fn reverse(&mut self, range: impl RangeBounds<usize>) {
        let Some((l, c, r)) = Node::split_at_3(self.root(), range) else { return; };
        c.rev ^= true; c.push();
        self.set_root(Node::merge_3(l, c, r));
    }
    
    /// `tree[range]` を畳み込んだ値を返す。区間が空のとき `None` を返す。
    pub fn fold(&self, range: impl RangeBounds<usize>) -> Option<[Ops::Acc; 2]> {
        let Some((l, c, r)) = Node::split_at_3(self.root(), range) else { return None; };
        c.update();
        let res = c.acc.clone();
        self.set_root(Node::merge_3(l, c, r));
        Some(res)
    }
    
    /// `tree[range]` に `lazy` を作用させる。
    pub fn act(&mut self, range: impl RangeBounds<usize>, lazy: Ops::Lazy) {
        let Some((l, c, r)) = Node::split_at_3(self.root(), range) else { return; };
        c.lazy = Some(lazy); c.push();
        self.set_root(Node::merge_3(l, c, r));
    }
    
    /// 指定した位置の参照を返す。`&self[idx]` は `self.get().unwrap()` と同等。
    /// 
    /// 範囲外のとき `None` を返す。
    pub fn get(&self, i: usize) -> Option<&Ops::Value> {
        if self.len() <= i { return None; }
        let root = Node::splay_index(Node::unwrap(self.root()), i);
        self.set_root(root);
        Some(&root.value)
    }
    
    /// 指定した位置の可変参照を返す。
    /// 
    /// 範囲外のとき `None` を返す。
    pub fn get_mut(&mut self, i: usize) -> Option<RefMut<'_, Ops>> {
        if self.len() <= i { return None; }
        let root = Node::splay_index(Node::unwrap(self.root()), i);
        self.set_root(root);
        Some(RefMut(root))
    }
    
    /// 木を右からマージする。
    pub fn merge(&mut self, right: Self) {
        self.set_root(Node::merge(self.root(), right.root()));
    }
    
    /// `self.into_iter() == self.iter()`
    pub fn iter(&self) -> Iter<'_, Ops> {
        Iter { splay: self, st: 0, ed: self.len() }
    }
    
    /// `range` 範囲の `Iterator` を返す。
    pub fn range(&self, range: impl RangeBounds<usize>) -> Iter<'_, Ops> {
        let [st, ed] = to_bounds(range, self.len());
        Iter { splay: self, st, ed }
    }
    
    /// `f(i-1) = true, f(i) = false` となる `i` を一つ返す。
    pub fn partition_point(&self, f: impl FnMut(&Ops::Value, &[Ops::Acc; 2]) -> bool) -> usize {
        let (i, ptr) = Node::partition_point(self.root(), f);
        self.set_root(ptr);
        i
    }
}

impl<Ops: SplayOp> FromIterator<Ops::Value> for SplayTree<Ops> {
    fn from_iter<T: IntoIterator<Item = Ops::Value>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let mut root = if let Some(v) = iter.next() { Node::new(v) } else { return Self::new(); };
        for v in iter {
            let node = Node::new(v);
            root.parent = node;
            node.child[0] = root;
            node.update();
            root = node;
        }
        Self(Cell::new(root))
    }
}

impl<'a, Ops: SplayOp> IntoIterator for &'a SplayTree<Ops> {
    type Item = &'a Ops::Value;
    type IntoIter = Iter<'a, Ops>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<Ops: SplayOp> Clone for SplayTree<Ops> {
    fn clone(&self) -> Self { self.iter().map(|v| v.clone()).collect() }
}

impl<Ops: SplayOp> Index<usize> for SplayTree<Ops> {
    type Output = Ops::Value;
    fn index(&self, index: usize) -> &Self::Output { self.get(index).expect("Out of index (@ SplayTree, Index<usize>)") }
}

impl<Ops: SplayOp> Debug for SplayTree<Ops> where Ops::Value: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<Ops: SplayOp> Drop for SplayTree<Ops> {
    fn drop(&mut self) {}
}



/// [`SplayTree::entry`] の返り値型。
/// 
/// # Constraints
/// 
/// `self.0` が `entry` 対象で、かつ根であること。
pub struct RefMut<'a, Ops: SplayOp>(&'a mut Node<Ops>);

impl<Ops: SplayOp> Deref for RefMut<'_, Ops> {
    type Target = Ops::Value;
    fn deref(&self) -> &Self::Target { &self.0.value }
}

impl<Ops: SplayOp> DerefMut for RefMut<'_, Ops> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0.value }
}

impl<Ops: SplayOp> Drop for RefMut<'_, Ops> {
    fn drop(&mut self) { self.0.update(); }
}



/// [`SplayTree::iter`], [`SplayTree::range`] の返り値型。保持する区間は `st..ed` で表される。
/// 
/// # Constraints
/// 
/// [`Iter`] 存在中に破壊的変更をしない。
pub struct Iter<'a, Ops: SplayOp> {
    splay: &'a SplayTree<Ops>,
    st: usize,
    ed: usize
}

impl<'a, Ops: SplayOp> Iterator for Iter<'a, Ops> {
    type Item = &'a Ops::Value;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.st == self.ed { return None; }
        self.st += 1;
        Some(&self.splay[self.st-1])
    }
}

impl<'a, Ops: SplayOp> DoubleEndedIterator for Iter<'a, Ops> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.st == self.ed { return None; }
        self.ed -= 1;
        Some(&self.splay[self.ed])
    }
}
*/





/// 遅延 `lazy, rev` の対象は自身を含まない。
struct Node<Op: SplayOp> {
    parent: Option<NodeRef<Op>>,
    child: [Option<NodeRef<Op>>; 2],
    value: Op::Value,
    acc: [Op::Acc; 2],
    lazy: Option<Op::Lazy>,
    rev: bool,
    cnt: [usize; 2]
}

struct NodeRef<Op: SplayOp>(NonNull<Node<Op>>);

fn connect<Op: SplayOp>(p: Option<NodeRef<Op>>, c: Option<NodeRef<Op>>, pos: bool) {
    if let Some(mut p) = p { p.child[pos as usize] = c; }
    if let Some(mut c) = c { c.parent = p; }
}

impl<Op: SplayOp> NodeRef<Op> {
    fn new(value: Op::Value) -> Self {
        let node = Node {
            parent: None,
            child: [None, None],
            acc: [Op::to_acc(&value), Op::to_acc(&value)],
            value,
            lazy: None,
            rev: false,
            cnt: [0, 0]
        };
        unsafe {
            Self(NonNull::new_unchecked(Box::into_raw(Box::new(node))))
        }
    }
    
    fn len(self) -> usize { 1 + self.cnt[0] + self.cnt[1] }
    
    /// parent の位置に self が来るよう回転する。
    fn rotate(self, p: Self, pos: bool) {
        connect(p.parent, Some(self), p.parent.map_or(false, |g| g.child[1] == Some(p)));
        connect(Some(p), self.child[!pos as usize], pos);
        connect(Some(self), Some(p), !pos);
        p.recalc();
        self.recalc();
    }
    
    /// `cnt`, `acc` を再計算する。
    fn recalc(mut self) {
        self.cnt = [0; 2];
        self.acc[0] = Op::to_acc(&self.value);
        self.acc[1] = self.acc[0].clone();
        for i in 0..2 {
            let Some(c) = self.child[i] else { continue; };
            self.cnt[i] = c.len();
            self.acc[i] = Op::prod_acc(&c.acc[i], &self.acc[i]);
            self.acc[i^1] = Op::prod_acc(&self.acc[i^1], &c.acc[i^1]);
        }
    }
    
    /// `self` を splay する。根から `self` が全て `push` されている必要がある。
    fn splay(self) {
        while let Some(p) = self.parent {
            let pos = p.child[1] == Some(self);
            let Some(pp) = p.parent else { self.rotate(p, pos); return; };
            if pos == (pp.child[1] == Some(p)) {
                // zig-zig
                p.rotate(pp, pos); self.rotate(p, pos);
            } else {
                // zig-zag
                self.rotate(p, pos); self.rotate(pp, !pos);
            }
        }
    }
    
    /// `lazy`, `rev` を解決・伝搬する。特に、子の `value, acc` は遅延が適用された状態になる。
    fn push(mut self) {
        if let Some(lazy) = self.lazy.take() {
            for c in self.child {
                let Some(mut c) = c else { continue; };
                Op::act_value(&mut c.value, &lazy);
                Op::act_acc(&mut c.acc[0], &lazy);
                Op::act_acc(&mut c.acc[1], &lazy);
                c.comp_lazy(&lazy);
            }
        }
        
        if replace(&mut self.rev, false) {
            for c in self.child {
                let Some(mut c) = c else { continue; };
                c.cnt.swap(0, 1);
                c.child.swap(0, 1);
                c.acc.swap(0, 1);
                c.rev ^= true;
            }
        }
    }
    
    fn comp_lazy(mut self, lazy: &Op::Lazy) {
        if let Some(l) = self.lazy.as_mut() {
            Op::comp_lazy(l, lazy);
        } else {
            self.lazy = Some(lazy.clone());
        }
    }
}






impl<Op: SplayOp> Deref for NodeRef<Op> {
    type Target = Node<Op>;
    fn deref(&self) -> &Self::Target { unsafe { self.0.as_ref() } }
}
impl<Op: SplayOp> DerefMut for NodeRef<Op> {
    fn deref_mut(&mut self) -> &mut Self::Target { unsafe { self.0.as_mut() } }
}
impl<Op: SplayOp> Clone for NodeRef<Op> {
    fn clone(&self) -> Self { Self(self.0) }
}
impl<Op: SplayOp> Copy for NodeRef<Op> {}
impl<Op: SplayOp> PartialEq for NodeRef<Op> {
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}
impl<Op: SplayOp> Eq for NodeRef<Op> {}



/*
/// Splay 木の `Node`
struct Node<Ops: SplayOps> {
    pub parent: *mut Self,
    pub child: [*mut Self; 2],
    pub len: usize,
    pub rev: bool,
    pub value: Ops::Value,
    pub acc: [Ops::Acc; 2],
    pub lazy: Option<Ops::Lazy>
}

impl<Ops: SplayOps> Node<Ops> {
    /// leak した `Node` を返す。
    fn new<'a>(value: Ops::Value) -> &'a mut Self {
        let acc = Ops::to_acc(&value);
        let tmp = Self { parent: null_mut(), child: [null_mut(); 2], len: 1, rev: false, value, acc: [acc.clone(), acc], lazy: None };
        Box::leak(Box::new(tmp))
    }
    
    fn get<'a>(ptr: *const Self) -> Option<&'a Self> { unsafe { ptr.as_ref() } }
    fn get_mut<'a>(ptr: *mut Self) -> Option<&'a mut Self> { unsafe { ptr.as_mut() } }
    fn unwrap<'a>(ptr: *mut Self) -> &'a mut Self { unsafe { &mut *ptr } }
    
    #[allow(dead_code)]
    fn debug(&self) -> String where Ops::Value: Debug, Ops::Acc: Debug, Ops::Lazy: Debug {
        let mut res = String::new();
        if let Some(l) = Node::get(self.child[0]) { res += &l.debug(); res += "\n"; }
        res += &format!("{:?}: par = {:?}, child = {:?}, len = {}, rev = {}, value = {:?}, acc = {:?}, lazy = {:?}", self as *const _, self.parent, self.child, self.len, self.rev, self.value, self.acc, self.lazy);
        if let Some(r) = Node::get(self.child[1]) { res += "\n"; res += &r.debug(); }
        res
    }
    
    /// 部分木の大きさを返す。
    fn len(ptr: *mut Self) -> usize { Node::get(ptr).map_or(0, |ptr| ptr.len) }
    
    /// 親 `p` に対する `self` の位置を返す。
    fn pos(&self, p: &Self) -> usize { ptr_eq(p.child[1], self) as usize }
    
    /// `self` が根になるよう、適切に回転させる。
    fn splay(&mut self) {
        while let Some(p) = Node::get_mut(self.parent) {
            let pos = self.pos(p);
            let Some(g) = Node::get_mut(p.parent) else { self.rotate(p, pos); return; };
            if pos == p.pos(g) {
                p.rotate(g, pos); self.rotate(p, pos);
            } else {
                self.rotate(p, pos); self.rotate(g, pos^1);
            }
        }
    }
    
    /// `self` が自分自身の親の位置に来るように回転させる。
    /// 
    /// `p.child[pos] = self` である必要がある。
    /// 
    /// # Memo
    /// 
    /// - `g --[pos]--> self`
    /// - `p --[pos]--> self.child[pos^1]`
    /// - `self --[pos^1]--> p`
    fn rotate(&mut self, p: &mut Self, pos: usize) {
        self.push();
        self.parent = p.parent;
        if let Some(g) = Node::get_mut(self.parent) { g.child[p.pos(g)] = self; }
        p.child[pos] = self.child[pos^1];
        if let Some(c) = Node::get_mut(p.child[pos]) { c.parent = p; }
        p.parent = self;
        self.child[pos^1] = p;
        p.update(); self.update();
    }
    
    /// 遅延作用と反転を行い、子に伝搬させる。
    fn push(&mut self) {
        if let Some(ad) = self.lazy.take() {
            Ops::act_value(&mut self.value, &ad);
            Ops::act_acc(&mut self.acc[0], &ad);
            Ops::act_acc(&mut self.acc[1], &ad);
            
            for c in self.child {
                let Some(Node { lazy, .. }) = Node::get_mut(c) else { continue; };
                if let Some(lazy) = lazy { Ops::comp_lazy(lazy, &ad); } else { *lazy = Some(ad.clone()); }
            }
        }
        
        if replace(&mut self.rev, false) {
            self.child.swap(0, 1);
            self.acc.swap(0, 1);
            for c  in self.child { if let Some(c) = Node::get_mut(c) { c.rev ^= true; } }
        }
    }
    
    /// `self.child` に対して [`Node::push`] してから `self.len, self.acc` を子の情報から更新する。
    fn update(&mut self) {
        self.len = 1;
        self.acc = { let tmp = Ops::to_acc(&self.value); [tmp.clone(), tmp] };
        for pos in [0, 1] {
            let Some(c) = Node::get_mut(self.child[pos]) else { continue; };
            c.push();
            self.len += c.len;
            self.acc[pos] = Ops::prod_acc(&c.acc[pos], &self.acc[pos]);
            self.acc[pos^1] = Ops::prod_acc(&self.acc[pos^1], &c.acc[pos^1]);
        }
    }
    
    /// 頂点 `i` を `splay` して返す。
    /// 
    /// # Constraints
    /// 
    /// `i < root.len`
    fn splay_index(mut root: &mut Self, mut i: usize) -> &mut Self {
        loop {
            // root.push();
            for c in root.child { if let Some(c) = Node::get_mut(c) { c.push(); } }
            let ls = Node::len(root.child[0]);
            
            use std::cmp::Ordering::*;
            root = match i.cmp(&ls) {
                Less => { Node::unwrap(root.child[0]) }
                Equal => { root.splay(); return root; }
                Greater => { i -= ls + 1; Node::unwrap(root.child[1]) }
            }
        }
    }
    
    /// マージした木を返す。
    /// 
    /// `l` の一番右の頂点が根になるよう `splay` した後に `r` と繋げる。
    fn merge(l: *mut Self, r: *mut Self) -> *mut Self {
        let Some(mut left) = Node::get_mut(l) else { return r; };
        let Some(right) = Node::get_mut(r) else { return left; };
        left = Self::splay_index(left, left.len-1);
        left.push();
        left.child[1] = right; right.parent = left;
        left.update();
        left
    }
    
    /// `tree[..at], tree[at..]` で構成される 2 つの木を返す。
    /// 
    /// # Constraints
    /// 
    /// `at <= root.len`
    /// 
    /// # Memo
    /// 
    /// `splay_index(at)` してから切る。
    fn split_at(root: *mut Self, at: usize) -> [*mut Self; 2] {
        let Some(mut root) = Node::get_mut(root) else { return [null_mut(), null_mut()]; };
        if at == 0 { return [null_mut(), root]; }
        if at == root.len { return [root, null_mut()]; }
        root = Self::splay_index(root, at);
        root.push();
        let left = Node::unwrap(replace(&mut root.child[0], null_mut()));
        left.parent = null_mut();
        root.update();
        [left, root]
    }
    
    /// `tree[..st], tree[st..ed], tree[ed..]` を返す。
    /// 
    /// `range` が空区間であるとき `None` を返す。
    /// 
    /// # Panic
    /// 
    /// `range` が `0..self.len` の範囲外であるとき。
    fn split_at_3<'a>(root: *mut Self, range: impl RangeBounds<usize>) -> Option<(*mut Self, &'a mut Self, *mut Self)> {
        let [st, ed] = to_bounds(range, Node::len(root));
        if st == ed { return None; }
        let [c, r] = Self::split_at(root, ed);
        let [l, c] = Self::split_at(c, st);
        Some((l, Node::unwrap(c), r))
    }
    
    fn merge_3(l: *mut Self, c: *mut Self, r: *mut Self) -> *mut Self {
        Self::merge(Self::merge(l, c), r)
    }
    
    /// `f(i-1) = true, f(i) = false` となる `i` を一つ探す。
    /// 
    /// `j = min(i, len-1)` として、`splay(j)` したあと `(i, Node[j] (root))` を返す。
    /// 
    /// ただし `f(-1) = true, f(len) = false` とする。
    fn partition_point(root: *mut Self, mut f: impl FnMut(&Ops::Value, &[Ops::Acc; 2]) -> bool) -> (usize, *mut Self) {
        let Some(mut now) = Node::get_mut(root) else { return (0, root); };
        let mut okng = [(0, null_mut()), (now.len, null_mut())];
        loop {
            let pos = !f(&now.value, &now.acc) as usize;
            if pos == 0 {
                okng[pos].0 += Node::len(now.child[pos]);
            } else {
                okng[pos].0 -= Node::len(now.child[pos]) + 1;
            }
            okng[pos].1 = now;
            let Some(c) = Node::get_mut(now.child[pos^1]) else { break; };
            c.update(); now = c;
        }
        
        let ptr = Node::get_mut(okng[1].1).unwrap_or_else(|| Node::unwrap(okng[0].1));
        ptr.splay();
        (okng[1].0, ptr)
    }
}

*/
