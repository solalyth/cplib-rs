use std::fmt::Debug;

/// 「なにもしない操作」を表す構造体。
pub struct Nop;



/// モノイド。単位元を持つ二項演算。
pub trait Monoid: Sized {
    type T: Clone + PartialEq + Debug;
    fn e() -> Self::T;
    fn prod(l: &Self::T, r: &Self::T) -> Self::T;
}



/// 群。単位元と逆元を持つ二項演算。
/// 
/// 可換群も `Group` で運用しているので注意。
pub trait Group {
    type T: Clone + PartialEq + Debug;
    fn e() -> Self::T;
    fn add(l: &Self::T, r: &Self::T) -> Self::T;
    fn inv(x: &Self::T) -> Self::T;
    
    fn sub(l: &Self::T, r: &Self::T) -> Self::T { Self::add(l, &Self::inv(r)) }
}

impl Group for Nop {
    type T = ();
    fn e() {}
    fn add(_: &(), _: &()) {}
    fn inv(_: &()) {}
}
