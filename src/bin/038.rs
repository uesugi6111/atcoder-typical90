#[rustfmt::skip]
mod io_pro {
    #[macro_export] macro_rules! input{(sc=$sc:expr,$($r:tt)*)=>{input_inner!{$sc,$($r)*}};($($r:tt)*)=>{let mut sc=io_pro::Scanner::new(std::io::stdin().lock());input_inner!{sc,$($r)*}};}
    #[macro_export] macro_rules! input_inner{($sc:expr)=>{};($sc:expr,)=>{};($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};}
    #[macro_export] macro_rules! read_value{($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()};($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};($sc:expr,Usize1)=>{read_value!($sc,usize)-1};($sc:expr,$t:ty)=>{$sc.next::<$t>()};}
    pub struct Scanner{s:Box<str>,input:std::iter::Peekable<std::str::SplitAsciiWhitespace<'static>>,}
    impl Scanner{
        pub fn new<R:std::io::Read>(mut reader:R)->Self{let mut sc=Scanner{s:{let mut s=String::new();reader.read_to_string(&mut s).unwrap();s.into_boxed_str()},input:"".split_ascii_whitespace().peekable(),};let s:&'static str=unsafe{std::mem::transmute(&*sc.s)};sc.input=s.split_ascii_whitespace().peekable();sc}
        #[inline]pub fn next<T:std::str::FromStr>(&mut self)->T where T::Err:std::fmt::Debug,{self.input.next().unwrap().parse::<T>().expect("Parse error")}
    }
}
#[proconio::fastout]
fn main() {
    input!(a: u64, b: u64);

    let mut ans = a as u128 * b as u128 / gcd(a, b) as u128;
    println!(
        "{}",
        if ans <= 10u128.pow(18) {
            ans.to_string()
        } else {
            "Large".to_string()
        }
    );
}

pub fn ngcd(m: u64, n: u64) -> u64 {
    if m == 0 {
        n
    } else {
        ngcd(n % m, m)
    }
}
pub fn lcm(m: u64, n: u64) -> u64 {
    m * n / gcd(m, n)
}
use std::cmp::min;
use std::mem::swap;
pub fn gcd(mut m: u64, mut n: u64) -> u64 {
    if m == 0 || n == 0 {
        return n;
    }
    let (i, j) = (
        // unsafe { std::num::NonZeroU64::new_unchecked(m) }.trailing_zeros(),
        // unsafe { std::num::NonZeroU64::new_unchecked(n) }.trailing_zeros(),
        m.trailing_zeros(),
        n.trailing_zeros(),
    );
    m >>= i;
    n >>= j;

    loop {
        if m > n {
            swap(&mut m, &mut n);
        }
        n -= m;
        if n == 0 {
            return m << min(i, j);
        }
        // n >>= unsafe { std::num::NonZeroU64::new_unchecked(n) }.trailing_zeros();
        n >>= n.trailing_zeros();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one() {
        assert_eq!(gcd(1, 2), 1);
        assert_eq!(gcd(2, 3), 1);
        assert_eq!(gcd(3, 5), 1);
        assert_eq!(gcd(5, 7), 1);
        assert_eq!(gcd(7, 9), 1);
        assert_eq!(gcd(9, 11), 1);
        assert_eq!(gcd(11, 13), 1);
    }
    #[test]
    fn t() {
        assert_eq!(gcd(2, 2), 2);
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(10, 15), 5);
        assert_eq!(gcd(6, 4), 2);
        assert_eq!(gcd(100, 30), 10);
        assert_eq!(gcd(1_000_000_008, 1_000_000_007), 1);
    }
}
