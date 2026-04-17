/// |Range|Chars|
/// |---|---|
/// |`33..48`|`!"#$%&'()*+,-./`|
/// |`48..58`|`0123456789`|
/// |`58..65`|`:;<=>?@`|
/// |`65..91`|`ABCDEFGHIJKLMNOPQRSTUVWXYZ`|
/// |`91..97`|``[\]^_` ``|
/// |`97..123`|`abcdefghijklmnopqrstuvwxyz`|
/// |`123..127`|`{\|}~`|
pub trait CharUtil: Clone {
    const LOWER: [Self; 26];
    const UPPER: [Self; 26];
    const NUMBER: [Self; 10];
    
    fn us(self) -> usize;
    fn parse_lower(self) -> usize;
    fn parse_upper(self) -> usize;
    fn parse_digit(self) -> usize;
    
    fn flip(self) -> Self;
}

impl CharUtil for char {
    const LOWER: [char; 26] = {
        let (mut out, mut i) = (['_'; 26], 0);
        while i < 26 { out[i] = (i+97) as u8 as char; i += 1; }
        out
    };
    
    const UPPER: [char; 26] = {
        let (mut out, mut i) = (['_'; 26], 0);
        while i < 26 { out[i] = (i+65) as u8 as char; i += 1; }
        out
    };
    
    const NUMBER: [char; 10] = {
        let (mut res, mut i) = (['_'; 10], 0);
        while i < 10 { res[i] = (i+48) as u8 as char; i += 1; }
        res
    };
    
    fn us(self) -> usize {
        if self <= '9' { self as usize - 48 } else { (self as usize & 31) - 1 }
    }
    fn parse_lower(self) -> usize { debug_assert!('a' <= self && self <= 'z'); self as usize - 97 }
    fn parse_upper(self) -> usize { debug_assert!('A' <= self && self <= 'Z'); self as usize - 65 }
    fn parse_digit(self) -> usize { debug_assert!('0' <= self && self <= '9'); self as usize - 48 }
    
    fn flip(self) -> Self { (self as u8 ^ 32) as char }
}
