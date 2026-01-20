pub fn replace_inf_and_truncate(mut s: String) -> String {
    let (mut res, mut stk) = (String::new(), String::new());
    s.push('*');
    for c in s.chars() {
        if c.is_numeric() || c == '.' { stk.push(c); continue; }
        // inf = 1.15e18 < 2^60
        if stk.parse::<f64>().map_or(false, |s| 1.15e18 <= s) { res += "inf"; } else { res += &stk; }
        stk.clear();
        res.push(c);
    }
    res.pop();
    if res.len() >= 500 { res.truncate(500); res += "<skipped>"; }
    res
}

pub fn epr_table<T: std::fmt::Debug>(src: &Vec<Vec<T>>, mut imax: usize, mut jmax: usize) {
    if crate::cplib::SUBMISSION { return; }
    
    use std::fmt::Write;
    let (mut lmax, mut res) = (2, String::from("     "));
    imax = imax.min(src.len());
    if 0 < imax { jmax = jmax.min(src[0].len()).min(100); }
    let tmp: Vec<Vec<String>> = (0..imax).map(|i| (0..jmax).map(|j| {
        let s = if let Some(x) = src[i].get(j) {
            replace_inf_and_truncate(format!("{x:?}"))
        } else {
            "--".into()
        };
        lmax = lmax.max(s.len());
        s
    }).collect()).collect();
    // if lmax != 0 { jmax = jmax.min(60/lmax); }
    for i in 0..jmax { write!(&mut res, "{i: >lmax$}  ").unwrap(); }
    res += "\n";
    for i in 0..imax {
        write!(&mut res, "{i: >2}: [").unwrap();
        for s in &tmp[i][..jmax] { write!(&mut res, "{s: >lmax$}, ").unwrap(); }
        res.pop(); res.pop(); res += "]\n";
    }
    
    eprintln!("\x1b[38;5;208m--- start epr_table ---\n{res}--- end epr_table ---\n\x1b[0m");
}



#[macro_export]
macro_rules! epr {
    ($($args:tt)*) => {
        if !$crate::SUBMISSION {
            eprintln!("\x1b[31m{}\x1b[0m", crate::util::debug::replace_inf_and_truncate(format!($($args)*)));
            // eprintln!("\x1b[31m{}\x1b[0m", format!($($args)*));
        }
    }
}


#[macro_export]
macro_rules! oj_local {
    ($oj:expr, $local:expr) => {
        if $crate::SUBMISSION { $oj } else { $local }
    };
}
