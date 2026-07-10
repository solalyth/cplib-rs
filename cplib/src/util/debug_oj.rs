pub fn replace_inf_and_truncate(s: String) -> String { s }
pub fn debug_table<T: std::fmt::Debug>(_: &Vec<Vec<T>>, _: usize, _: usize) {}



#[macro_export]
macro_rules! epr {
    ($($args:tt)*) => {}
}

#[macro_export]
macro_rules! oj_local {
    ($oj:expr, $local:expr) => {
        if $crate::LOCAL { $local } else { $oj }
    };
}

#[macro_export]
macro_rules! table {
    ($t:expr, $x:expr, $y:expr) => {};
    ($t:expr) => {};
}
