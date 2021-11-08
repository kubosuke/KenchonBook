use {
    proconio::input,
    proconio::marker::Chars,
};

macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_min = min!($($cmps),+);
        if $base > cmp_min {
            $base = cmp_min;
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

const INF: i32 = 2 << 20;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut dp: Vec<Vec<i32>> = vec![vec![INF; t.len() + 1]; s.len() + 1];

    // initialize
    dp[0][0] = 0;

    for i in 0..=s.len() {
        for j in 0..=t.len() {
            // modification
            if i > 0 && j > 0 {
                if s[i-1] == t[j-1] {
                    chmin!(dp[i][j], dp[i-1][j-1])
                } else {
                    chmin!(dp[i][j], dp[i-1][j-1] + 1)
                }
            }
            // delete
            if i > 0 {
                chmin!(dp[i][j], dp[i-1][j] + 1)
            }
            // insert
            if j > 0 {
                chmin!(dp[i][j], dp[i][j-1] + 1)
            }
        }
    }
    println!("{}", dp[s.len()][t.len()]);
}
