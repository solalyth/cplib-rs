#[macro_export]
macro_rules! init {
    ($iter:ident) => {
        let _s = std::io::read_to_string(std::io::stdin()).unwrap();
        let mut $iter = _s.split_whitespace();
    };
}

#[macro_export]
macro_rules! input {
    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().unwrap()
    };
}
