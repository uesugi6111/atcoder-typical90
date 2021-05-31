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
    input!(
        n: usize,
        cp: [(i64, i64); n],
        q: usize,
        lr: [(usize, usize); q]
    );

    let cumsum_1 = (0..1)
        .chain(
            cp.iter()
                .map(|&(c, p)| if c == 1 { p } else { 0 })
                .scan(0, |c, x| {
                    *c += x;
                    Some(*c)
                }),
        )
        .collect::<Vec<_>>();
    let cumsum_2 = (0..1)
        .chain(
            cp.iter()
                .map(|&(c, p)| if c == 2 { p } else { 0 })
                .scan(0, |c, x| {
                    *c += x;
                    Some(*c)
                }),
        )
        .collect::<Vec<_>>();

    for (l, r) in lr {
        println!(
            "{} {}",
            cumsum_1[r] - cumsum_1[l - 1],
            cumsum_2[r] - cumsum_2[l - 1]
        );
    }
}
