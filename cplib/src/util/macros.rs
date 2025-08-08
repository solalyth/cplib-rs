#[macro_export]
macro_rules! epr {
    ($($args:tt)*) => {
        if !$crate::SUBMISSION {
            #[allow(unused_mut)]
            let tmp = format!($($args)*);
            let tmp = tmp.replace("18446744073709551615", "MAX").replace("9223372036854775807", "MAX");
            // if 500 < tmp.len() {
            //     tmp.truncate(500); tmp += "...";
            // }
            eprintln!("\x1b[31m{tmp}\x1b[0m");
        }
    }
}

#[macro_export]
macro_rules! table {
    ($v:expr) => {
        if !$crate::SUBMISSION {
            use std::fmt::Write;
            let mut lmax = $v.iter().map(|v| v.iter().map(|e| format!("{e}").len()).max().unwrap_or(0)).max().unwrap_or(0);
            let mut tmp = String::new();
            for v in &$v {
                tmp += "    [ ";
                for &e in v {
                    write!(&mut tmp, "{e: >lmax$}, ");
                }
                tmp.pop(); tmp.pop();
                tmp += "]\n";
            }
            eprintln!("\x1b[38;5;208m{} = [\n{tmp}]\x1b[0m", stringify!($v));
        }
    };
    ($v:expr, $inf:expr) => {
        if !$crate::SUBMISSION {
            use std::fmt::Write;
            let mut lmax = $v.iter().map(|v| v.iter().map(|e| format!("{e}").len()).max().unwrap_or(0)).max().unwrap_or(0).max(3);
            let mut tmp = String::new();
            for v in &$v {
                tmp += "    [ ";
                for &e in v {
                    if e < $inf {
                        write!(&mut tmp, "{e: >lmax$}, ");
                    } else {
                        write!(&mut tmp, "{: >lmax$}, ", "inf");
                    }
                }
                tmp.pop(); tmp.pop();
                tmp += "]\n";
            }
            eprintln!("\x1b[38;5;208m{} = [\n{tmp}]\x1b[0m", stringify!($v));
        }
    };
}

#[macro_export]
macro_rules! oj_local {
    ($oj:expr, $local:expr) => {
        if $crate::SUBMISSION { $oj } else { $local }
    };
}



/// `nest!(void; 2; 3) = vec[0..2][0..3]: [[vec![]; 3]; 2]`
/// 
/// `nest!(e; 2; 3) = vec[0..2][0..3]: [[e; 3]; 2]`
#[macro_export]
macro_rules! vec {
    [void; $n:expr] => { std::vec![std::vec![]; $n] };
    [void; $n:expr $(;$m:expr)+] => { std::vec![vec![void$(;$m)+]; $n] };
    
    // [] => { std::vec![] };
    [$($v:expr),*] => { std::vec![$($v),*] };
    [$e:expr; $n:expr] => { std::vec![$e; $n] };
    [$e:expr; $n:expr $(;$m:expr)+] => { std::vec![vec![$e$(;$m)+]; $n] };
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

/// `values < dst` であるとき `true` を返す。
#[macro_export]
macro_rules! chmin {
    ($dst:expr; $v:expr) => { { let v = $v; if v < $dst { $dst = v; true } else { false } } };
    ($dst:expr; $($vl:expr),+) => { crate::chmin!($dst; crate::min!($($vl),+)) }
}

/// `dst < values` であるとき `true` を返す。
#[macro_export]
macro_rules! chmax {
    ($dst:expr; $v:expr) => { { let v = $v; if $dst < v { $dst = v; true } else { false } } };
    ($dst:expr; $($vl:expr),+) => { crate::chmax!($dst; crate::max!($($vl),+)) }
}


/// `0^0 == 1` とする。
#[macro_export]
macro_rules! safe_pow {
    ($v:expr, $e:expr) => { {
        let (mut v, mut e, mut res) = ($v, $e, 1);
        if e == 0 {1} else if v == 0 {0} else {
            while e != 0 {
                if e%2 == 1 {
                    res = res.saturating_mul(v);
                }
                v = v.saturating_mul(v);
                e /= 2;
            }
            res
        }
    } };
    ($v:expr, $e:expr, $m:expr) => { {
        let (mut v, mut e, m, mut res) = ($v, $e, $m, 1);
        if e == 0 {1%m} else if v == 0 {0} else {
            while e != 0 {
                if e%2 == 1 {
                    res = (res*v)%m;
                }
                v = v*v%m;
                e /= 2;
            }
            res.rem_euclid(m)
        }
    } }
}

/// `map_get(map, key, def)` -> `map.entry(key).or_insert(def): &mut V`
#[macro_export]
macro_rules! map_get {
    ($map:expr, $key:expr, $def:expr) => {
        $map.entry($key).or_insert($def)
    };
}
