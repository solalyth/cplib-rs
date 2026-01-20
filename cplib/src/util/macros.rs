/// `nest!(void; 2; 3) = vec[0..2][0..3]: [[vec![]; 3]; 2]`
/// 
/// `nest!(e; 2; 3) = vec[0..2][0..3]: [[e; 3]; 2]`
#[macro_export]
macro_rules! nest {
    [void; $n:expr] => { std::vec![std::vec![]; $n] };
    [void; $n:expr $(;$m:expr)+] => { std::vec![crate::nest![void$(;$m)+]; $n] };
    
    [$($v:expr),*] => { std::vec![$($v),*] };
    [$e:expr; $n:expr] => { std::vec![$e; $n] };
    [$e:expr; $n:expr $(;$m:expr)+] => { std::vec![crate::nest![$e$(;$m)+]; $n] };
}

#[macro_export]
macro_rules! iota {
    ($range:expr) => { ($range).collect::<Vec<_>>() };
    ($range:expr, $($f:tt)*) => { ($range).map($($f)*).collect::<Vec<_>>() };
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

#[macro_export]
macro_rules! minmax {
    ($v:expr, $w:expr) => {{
        let (v, w) = ($v, $w); if v <= w { (v, w) } else { (w, v) }
    }};
    ($($vl:expr),+) => {{
        let l = [$($vl),+]; (l.iter().reduce(|x,y| if x <= y {x} else {y}).unwrap().clone(), l.iter().reduce(|x,y| if x >= y {x} else {y}).unwrap().clone())
    }}
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

#[macro_export]
macro_rules! swap {
    ($l:expr, $r:expr) => { ($l, $r) = ($r, $l); };
}

#[macro_export]
macro_rules! prefix {
    ($v:expr) => { {
        let mut res = vec![0];
        for x in $v.into_iter() { res.push(res.last().unwrap()+x); }
        res
    } };
}

#[macro_export]
macro_rules! vadd {
    ($v:expr, -$x:expr) => {{
        let x = $x;
        for e in &mut $v { *e -= x; }
    }};
    ($v:expr, $x:expr) => {{
        let x = $x;
        for e in &mut $v { *e += x; }
    }}
}
