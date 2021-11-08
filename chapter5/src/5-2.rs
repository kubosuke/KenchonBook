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
        N: usize,
        W: isize,
        a: [isize; N],
    }
    // dp[I][J]: use 0..I and whether summation can be J
    let mut dp: Vec<Vec<bool>> = vec![vec![false; N+1]; (W + 1) as usize];

    // init
    for i in 0..=N {
        dp[i][0] = true
    }

    for i in 0..N {
        for w in 0..=W {
            if w - a[i] >= 0 {
                dp[i+1][w as usize] = dp[i+1][w as usize] || dp[i][(w - a[i]) as usize]
            }
            dp[i+1][w as usize] = dp[i+1][w as usize] || dp[i][w as usize]
        }
    }
    println!("{}", dp[N][W as usize])
}
