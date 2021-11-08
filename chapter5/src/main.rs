// Q: https://atcoder.jp/contests/dp/tasks/dp_f
// ref: https://www.coursera.org/lecture/dynamic-programming-greedy-algorithms/dynamic-programming-longest-common-subsequence-H3XVF

use proconio::input;
use proconio::marker::Chars;

#[allow(unused_macros)]
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

#[allow(unused_macros)]
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

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let m = s.len();
    let n = t.len();
    let mut dp = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            if s[i] == t[j] && i > 0 && j > 0 {
                chmax!(dp[i][j], dp[i-1][j-1] + 1);
            }
            if s[i] == t[j] && (i == 0 || j == 0) {
                dp[i][j] = 1
            }
            if i > 0 {
                chmax!(dp[i][j], dp[i-1][j]);
            }
            if j > 0 {
                chmax!(dp[i][j], dp[i][j-1]);
            }
        }
    }
    println!("{}", dp[m-1][n-1])
}
