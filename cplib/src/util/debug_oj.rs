pub fn replace_inf_and_truncate(_: String) -> String { unimplemented!() }

pub fn epr_table<T: std::fmt::Debug>(_: &Vec<Vec<T>>, _: usize, _: usize) {}



#[macro_export]
macro_rules! epr {
    ($($args:tt)*) => {}
}

#[macro_export]
macro_rules! oj_local {
    ($oj:expr, $local:expr) => {
        if $crate::SUBMISSION { $oj } else { $local }
    };
}
