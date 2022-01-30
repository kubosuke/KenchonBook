// https://atcoder.jp/contests/dp/tasks/dp_g

use proconio::input;
use std::collections::HashSet;

macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_max = max!($($cmps),+);
        if $base < cmp_max {
            $base = cmp_max;
            true
        } else {
            false
        }
    }};
}

macro_rules! max {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::max($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::max($a, max!($($rest),+))
    }};
}

fn rec(v: usize, graph: &Vec<HashSet<usize>>, dp: &mut Vec<isize>) -> isize {
    if dp[v] != -1 {
        return dp[v];
    }
    let mut res = 0;
    for vv in &graph[v] {
        chmax!(res, rec(*vv, graph, dp) + 1);
    }
    dp[v] = res;
    res
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut paths: [(usize, usize); m],
    }

    let graph: Vec<HashSet<usize>> = {
        let mut tmp: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        for path in paths.iter() {
            tmp[path.0 - 1].insert(path.1 - 1); // 0-index
        }
        tmp
    };

    // dp[v]: length of the longest path from v
    let mut dp = vec![-1; n];

    let mut ans = 0;
    for v in 0..n {
        chmax!(ans, rec(v, &graph, &mut dp));
    }
    println!("{}", ans);
}
