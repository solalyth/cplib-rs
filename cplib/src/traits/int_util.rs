// pub fn floor<T: IntUtil>(l: T, r: T) -> T { l.floor(r).0 }
// pub fn floor_rem<T: IntUtil>(l: T, r: T) -> T { l.floor(r).1 }
pub fn ceil<T: IntUtil>(l: T, r: T) -> T { l.ceil(r).0 }
pub fn ceil_rem<T: IntUtil>(l: T, r: T) -> T { l.ceil(r).1 }


pub trait IntUtil: Sized {
    fn floor(self, rhs: Self) -> (Self, Self);
    fn ceil(self, rhs: Self) -> (Self, Self);
    fn width(self) -> Option<usize>;
}

macro_rules! int {
    ($($t:ty),+) => {$(
        impl IntUtil for $t {
            fn floor(self, rhs: Self) -> ($t, $t) {
                assert!(0 < rhs);
                let (q, r) = (self/rhs, self%rhs);
                if r < 0 { (q-1, r+rhs) } else { (q, r) }
            }
            
            fn ceil(self, rhs: Self) -> ($t, $t) {
                assert!(0 < rhs);
                let (q, r) = (self/rhs, self%rhs);
                if 0 < r { (q+1, rhs-r) } else { (q, -r) }
            }
            
            fn width(self) -> Option<usize> { unimplemented!() }
        }
    )+};
}

int!(i32, i64, i128, isize);

macro_rules! uint {
    ($($t:ty),+) => {$(
        impl IntUtil for $t {
            fn floor(self, rhs: Self) -> ($t, $t) {
                (self/rhs, self%rhs)
            }

            fn ceil(self, rhs: Self) -> ($t, $t) {
                let q = (self+rhs-1)/rhs;
                (q, q*rhs-self)
            }
            
            fn width(self) -> Option<usize> { if self == 0 {None} else {Some((Self::BITS-self.leading_zeros()-1) as usize)} }
        }
    )+};
}

uint!(u32, u64, u128, usize);
