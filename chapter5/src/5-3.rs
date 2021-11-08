// Q: https://atcoder.jp/contests/tdpc/tasks/tdpc_contest

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

const MAX_SUM: usize = 10001;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    // dp[i]: iとなる部分和が取れるかどうか
    let mut dp = vec![false; MAX_SUM];
    dp[0] = true;

    for pi in p {
        for i in (0..MAX_SUM).rev() {
            if dp[i] && i+pi < MAX_SUM {
                dp[i+pi] = true;
            }
        }
    }
    println!("{}", dp.iter().filter(|&&v|v).count())
}
