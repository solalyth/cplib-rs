/// `nest!(void; 2; 3) = vec[0..2][0..3]: [[vec![]; 3]; 2]`
/// 
/// `nest!(e; 2; 3) = vec[0..2][0..3]: [[e; 3]; 2]`
#[macro_export]
macro_rules! nest {
    [void; $n:expr] => { std::vec![std::vec![]; $n] };
    [void; $n:expr $(;$m:expr)+] => { std::vec![crate::nest![void$(;$m)+]; $n] };
    
    // [] => { std::vec![] };
    [$($v:expr),*] => { std::vec![$($v),*] };
    [$e:expr; $n:expr] => { std::vec![$e; $n] };
    [$e:expr; $n:expr $(;$m:expr)+] => { std::vec![crate::nest![$e$(;$m)+]; $n] };
}


// Float は Ord が使えないので reduce している

#[macro_export]
macro_rules! min {
    ($($vl:expr),+) => { [$($vl),+].into_iter().reduce(|x,y| if x <= y {x} else {y}).unwrap() }
}

#[macro_export]
macro_rules! max {
    ($($vl:expr),+) => { [$($vl),+].into_iter().reduce(|x,y| if x >= y {x} else {y}).unwrap() }
}

/// `min(values) < dst` であるとき `true` を返す。
#[macro_export]
macro_rules! chmin {
    ($dst:expr; $v:expr) => { { let v = $v; if v < $dst { $dst = v; true } else { false } } };
    ($dst:expr; $($vl:expr),+) => { crate::chmin!($dst; crate::min!($($vl),+)) }
}

/// `dst < max(values)` であるとき `true` を返す。
#[macro_export]
macro_rules! chmax {
    ($dst:expr; $v:expr) => { { let v = $v; if $dst < v { $dst = v; true } else { false } } };
    ($dst:expr; $($vl:expr),+) => { crate::chmax!($dst; crate::max!($($vl),+)) }
}
