// https://atcoder.jp/contests/abc051/tasks/abc051_d

use proconio::input;
use proconio::marker::Usize1;
const INF: usize = 18446744073709551615 / 3;

macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_min = min!($($cmps),+);
        if $base > cmp_min {
            $base = cmp_min;
            true
        } else {
            false
        }
    }};
}

macro_rules! min {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::min($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::min($a, min!($($rest),+))
    }};
}

fn main() {
    input! {
        n: usize, m: usize,
    }

    // Floyd-Warshal
    // Init
    let mut dp = vec![vec![INF; n]; n];
    let mut edges = vec![];
    for _ in 0..m {
        input! {
            a: Usize1, b: Usize1, w: usize
        }
        edges.push((a, b, w));
        dp[a][b] = w;
        dp[b][a] = w;
    }
    for i in 0..n {
        dp[i][i] = 0;
    }

    // DP
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                chmin!(dp[i][j], dp[i][k] + dp[k][j]);
            }
        }
    }

    // Result
    let mut res = 0;
    for e in edges {
        if dp[e.0][e.1] < e.2 {
            res += 1;
        }
    }
    println!("{}", res);
}
