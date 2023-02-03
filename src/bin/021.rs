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
    input!(n: usize, m: usize, ab: [(usize, usize); m]);

    let mut e = vec![vec![]; n];
    for (a, b) in ab {
        e[a - 1].push(b - 1);
    }
    let d = scc::decompose(&e);
    let ans = d.iter().map(|x| x.len() * (x.len() - 1) / 2).sum::<usize>();

    println!("{}", ans);
}

mod scc {
    #[derive(Debug)]
    enum Vertex {
        In(usize),
        Out(usize),
    }

    pub fn decompose(e: &[Vec<usize>]) -> Vec<Vec<usize>> {
        let mut seen = vec![false; e.len()];

        let mut stack = vec![];
        let mut nodes = Vec::with_capacity(e.len());
        for i in 0..e.len() {
            if seen[i] {
                continue;
            }
            stack.push(Vertex::In(i));

            while let Some(vertex) = stack.pop() {
                if let Vertex::In(v) = vertex {
                    if seen[v] {
                        continue;
                    }
                    stack.push(Vertex::Out(v));
                    seen[v] = true;
                    for &to in e[v].iter() {
                        stack.push(Vertex::In(to));
                    }
                } else if let Vertex::Out(v) = vertex {
                    nodes.push(v);
                }
            }
        }
        let mut reverse_edge = vec![vec![]; e.len()];
        for i in 0..e.len() {
            for j in 0..e[i].len() {
                reverse_edge[e[i][j]].push(i);
            }
        }

        let mut components = vec![];
        let mut back_stack = vec![];
        let mut back_seen = vec![false; e.len()];
        while let Some(v) = nodes.pop() {
            if back_seen[v] {
                continue;
            }
            let mut scc = vec![];
            back_stack.push(v);
            back_seen[v] = true;

            while let Some(v) = back_stack.pop() {
                for &to in reverse_edge[v].iter() {
                    if back_seen[to] {
                        continue;
                    }
                    back_stack.push(to);
                    back_seen[to] = true;
                }

                scc.push(v);
            }
            components.push(scc);
        }
        components
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_scc() {
            let n = 6;
            let v = vec![(1, 4), (5, 2), (3, 0), (5, 5), (4, 1), (0, 3), (4, 2)];
            let mut e = vec![vec![]; n];
            for &(v, u) in v.iter() {
                e[v].push(u);
            }
            let a = decompose(&e);
            assertvv(&a, &[vec![5], vec![1, 4], vec![2], vec![0, 3]]);
        }
        #[test]
        fn test_scc2() {
            let n = 7;
            let v = vec![
                (0, 2),
                (1, 2),
                (2, 3),
                (3, 2),
                (3, 4),
                (4, 5),
                (5, 6),
                (6, 4),
            ];
            let mut e = vec![vec![]; n];
            for &(v, u) in v.iter() {
                e[v].push(u);
            }
            let a = decompose(&e);
            dbg!(&a);
            assertvv(&a, &[vec![0], vec![1], vec![3, 2], vec![5, 6, 4]]);
        }

        #[test]
        fn test_scc3() {
            let n = 11;
            let v = vec![
                (0, 1),
                (1, 2),
                (1, 10),
                (2, 0),
                (2, 3),
                (3, 4),
                (4, 5),
                (4, 10),
                (5, 6),
                (6, 3),
                (7, 8),
                (7, 9),
                (8, 10),
                (9, 7),
                (9, 7),
                (9, 7),
                (9, 7),
                (10, 7),
            ];
            let mut e = vec![vec![]; n];
            for &(v, u) in v.iter() {
                e[v].push(u);
            }
            let a = decompose(&e);
            dbg!(&a);
            assertvv(&a, &[vec![0, 1, 2], vec![3, 4, 5, 6], vec![7, 8, 9, 10]]);
        }
        #[test]
        fn test_scc4() {
            let n = 5;
            let v = vec![(0, 1), (0, 2), (2, 3), (3, 4)];
            let mut e = vec![vec![]; n];
            for &(v, u) in v.iter() {
                e[v].push(u);
            }
            let a = decompose(&e);
            dbg!(&a);
            assertvv(&a, &[vec![0], vec![1], vec![2], vec![3], vec![4]]);
        }
        #[test]
        fn test_scc5() {
            let n = 5;
            let v = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)];
            let mut e = vec![vec![]; n];
            for &(v, u) in v.iter() {
                e[v].push(u);
            }
            let a = decompose(&e);
            dbg!(&a);
            assertvv(&a, &[vec![0, 1, 2, 3, 4]]);
        }
        #[test]
        fn test_scc6() {
            let n = 6;
            let v = vec![(0, 1), (1, 2), (2, 0), (0, 3), (3, 4), (4, 0)];
            let mut e = vec![vec![]; n];
            for &(v, u) in v.iter() {
                e[v].push(u);
            }
            let a = decompose(&e);
            dbg!(&a);
            assertvv(&a, &[vec![0, 1, 2, 3, 4], vec![5]]);
        }
        use std::collections::HashSet;
        fn assertvv(a: &[Vec<usize>], b: &[Vec<usize>]) -> Option<()> {
            assert_eq!(a.len(), b.len());
            let mut a = convert(a);
            let mut b = convert(b);

            for i in a.iter_mut() {
                for j in b.iter_mut() {
                    if j.is_none() {
                        continue;
                    }
                    if i.as_ref()?.eq(j.as_ref()?) {
                        i.take();
                        j.take();
                        break;
                    }
                }
                assert!(i.is_none());
            }
            Some(())
        }
        fn convert(a: &[Vec<usize>]) -> Vec<Option<HashSet<usize>>> {
            a.iter()
                .map(|x| Some(x.iter().cloned().collect::<HashSet<_>>()))
                .collect::<Vec<_>>()
        }
    }
}
