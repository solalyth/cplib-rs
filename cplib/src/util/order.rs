pub struct Order<const REV: bool, T: PartialOrd, U>(pub T, pub U);

#[macro_export]
macro_rules! pair {
    ($t:expr) => { crate::util::order::Order($t, ()) };
    ($f:literal; $t:expr) => { crate::util::order::Order::<$f, _, _>($t, ()) };
    ($t:expr, $u:expr) => { crate::util::order::Order($t, $u) };
    ($f:literal; $t:expr, $u:expr) => { crate::util::order::Order::<$f, _, _>($t, $u) };
}

// impl PartialEq, PartialOrd, Eq, Ord, Clone, Debug
macro_rules! a {
    ($x:ty; $($y:tt)*) => {
        impl<const REV: bool, T: PartialOrd, U> $x for Order<REV, T, U> { $($y)* }
    };
    ($x:ty, $z:tt; $($y:tt)*) => {
        #[allow(unused)] use std::fmt::*;
        impl<const REV: bool, T: PartialOrd+$z, U: $z> $x for Order<REV, T, U> { $($y)* }
    };
}
a!(PartialEq; fn eq(&self, other: &Self) -> bool { self.0 == other.0 });
a!(PartialOrd; fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { if REV { other.0.partial_cmp(&self.0) } else { self.0.partial_cmp(&other.0) } });
a!(Eq; );
a!(Ord; fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.partial_cmp(other).unwrap() });
a!(Clone, Clone; fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) });
a!(Debug, Debug; fn fmt(&self, f: &mut Formatter<'_>) -> Result { if std::any::type_name::<U>() == "()" { write!(f, "{:?}", self.0) } else { write!(f, "({:?}, {:?})", self.0, self.1) } });
