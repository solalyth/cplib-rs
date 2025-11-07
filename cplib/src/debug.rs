pub fn replace_inf(s: &str) -> String {
    let (mut res, mut stk) = (String::new(), String::new());
    for c in s.chars().chain(std::iter::once('*')) {
        if c.is_numeric() || c == '.' { stk.push(c); continue; }
        // inf = 1.15e18 < 2^60
        if stk.parse::<f64>().map_or(false, |s| 1.15e18 <= s) { res += "inf"; } else { res += &stk; }
        stk.clear();
        res.push(c);
    }
    res.pop();
    res
}



#[macro_export]
macro_rules! epr {
    ($($args:tt)*) => {
        if !$crate::SUBMISSION {
            eprintln!("\x1b[31m{}\x1b[0m", crate::debug::replace_inf(&format!($($args)*)));
            // eprintln!("\x1b[31m{}\x1b[0m", format!($($args)*));
        }
    }
}

#[macro_export]
macro_rules! table {
    ($vv:expr) => {
        if !$crate::SUBMISSION {
            use std::fmt::Write;
            let (mut lmax, mut res) = (0, String::new());
            let vv: Vec<Vec<String>> = $vv.iter().map(|v| v.iter().map(|e| { let t = crate::debug::replace_inf(&format!("{e:?}")); lmax = lmax.max(t.len()); t }).collect()).collect();
            for v in &vv {
                res += "    [ ";
                for e in v { write!(&mut res, "{e: >lmax$}, "); }
                res.pop(); res.pop(); res += "]\n";
            }
            eprintln!("\x1b[38;5;208m{} = [\n{res}]\x1b[0m", stringify!($vv));
        }
    };
}

#[macro_export]
macro_rules! oj_local {
    ($oj:expr, $local:expr) => {
        if $crate::SUBMISSION { $oj } else { $local }
    };
}
