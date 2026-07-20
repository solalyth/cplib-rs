/// `nest!(void; 2; 3) = vec[0..2][0..3]: [[vec![]; 3]; 2]`
/// 
/// `nest!(e; 2; 3) = vec[0..2][0..3]: [[e; 3]; 2]`
#[macro_export]
macro_rules! nest {
    [void; $n:expr] => { std::vec![std::vec![]; $n] };
    // void が打ちやすくて void になってしまっているが、変すぎる。
    [void; $n:expr $(;$m:expr)+] => { std::vec![crate::nest![void$(;$m)+]; $n] };
    
    [($ev:expr) for $i:ident in $rg:expr] => { $rg.into_iter().map(|$i| $ev).collect::<Vec<_>>() };
    
    [$($v:expr),*] => { std::vec![$($v),*] };
    [$e:expr; $n:expr] => { std::vec![$e; $n] };
    [$e:expr; $n:expr $(;$m:expr)+] => { std::vec![crate::nest![$e $(;$m)+]; $n] };
}

#[macro_export]
macro_rules! min {
    ($x:expr, $y:expr) => {{ let (x, y) = ($x, $y); if x <= y { x } else { y } }};
    // PartialOrd 用に reduce している
    ($($vl:expr),+) => { [$($vl),+].into_iter().reduce(|x,y| if x <= y {x} else {y}).unwrap() }
}

#[macro_export]
macro_rules! max {
    ($($vl:expr),+) => { [$($vl),+].into_iter().reduce(|x,y| if x >= y {x} else {y}).unwrap() }
}

/// `min(values) < dst` であるとき `true` を返す。
#[macro_export]
macro_rules! chmin {
    ($dst:expr; $v:expr) => {{ let v = $v; if v < $dst { $dst = v; true } else { false } }};
    ($dst:expr; $($vl:expr),+) => { crate::chmin!($dst; crate::min!($($vl),+)) }
}

/// `dst < max(values)` であるとき `true` を返す。
#[macro_export]
macro_rules! chmax {
    ($dst:expr; $v:expr) => {{ let v = $v; if $dst < v { $dst = v; true } else { false } }};
    ($dst:expr; $($vl:expr),+) => { crate::chmax!($dst; crate::max!($($vl),+)) }
}

#[macro_export]
macro_rules! swap {
    ($l:expr, $r:expr) => { ($l, $r) = ($r, $l); };
}

/// 累積和を求める。usage: `prefix!(init: T, iter: impl IntoIterator<T>)`, `prefix!(iter: impl IntoIterator<int or &int>)`
#[macro_export]
macro_rules! prefix {
    ($init:expr, $v:expr) => {{
        let mut res = vec![$init];
        for x in $v.into_iter() { res.push(*res.last().unwrap()+x); }
        res
    }};
    ($v:expr) => { prefix!(0, $v) }
}

#[macro_export]
/// 総和を求める。`sum!(init, iter)`, `sum!(iter)`
macro_rules! sum {
    ($init:expr, $v:expr) => {{
        let mut res = $init;
        for x in $v.into_iter() { res += x; }
        res
    }};
    ($v:expr) => { sum!(0, $v) }
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

/// for modint
#[macro_export]
macro_rules! o {
    ($x:expr, += $y:expr) => { let t = $y; $x = $x+t; };
    ($x:expr, -= $y:expr) => { let t = $y; $x = $x-t; };
    ($x:expr, *= $y:expr) => { let t = $y; $x = $x*t; };
    ($x:expr, /= $y:expr) => { let t = $y; $x = $x/t; };
}



/// `map_init!(map, key, value) -> &mut V`
#[macro_export]
macro_rules! map_init {
    ($map:expr, $key:expr, $value:expr) => { $map.entry($key).or_insert_with(|| $value) };
}

/// `counter!(map, key, x) -> int`
#[macro_export]
macro_rules! counter {
    ($map:expr, $key:expr, $x:expr) => {{
        let k = $key;
        let x = $x + $map.get(&k).unwrap_or(&0);
        if 0 < x { $map.insert(k, x); } else { $map.remove(&k); }
        x
    }}
}
