// Q: https://atcoder.jp/contests/dp/tasks/dp_c

use proconio::input;

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
        n: usize,
        abc: [(usize, usize, usize); n],
    }
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 3]; n + 1 ];

    for i in 0..=2 {
        dp[0][i] = 0
    }
    for i in 0..n {
        dp[i+1][0] = abc[i].0 + max!(dp[i][1], dp[i][2]);
        dp[i+1][1] = abc[i].1 + max!(dp[i][0], dp[i][2]);
        dp[i+1][2] = abc[i].2 + max!(dp[i][0], dp[i][1]);
    }
    println!("{}", dp[n].iter().max().unwrap())
}
