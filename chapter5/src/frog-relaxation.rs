#[allow(unused_imports)]
use {
    proconio::input,
    std::{cmp::*, collections::*, io::*, num::*, str::*},
};

#[allow(unused_macros)]
macro_rules! chmin {($base:expr, $($cmps:expr),+ $(,)*) => {{let cmp_min = min!($($cmps),+);if $base > cmp_min {$base = cmp_min;true} else {false}}};}
#[allow(unused_macros)]
macro_rules! min {
    ($a:expr $(,)*) => {{$a}};
    ($a:expr, $b:expr $(,)*) => {{if $a > $b {$b} else {$a}}};
    ($a:expr, $($rest:expr),+ $(,)*) => {{let b = min!($($rest),+);if $a > b {b} else {$a}}};
}

const INF: i64 = 2 << 60;

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    }
    let mut dp: Vec<i64> = vec![INF; n];
    dp[0] = 0;
    for i in 1usize..n {
        chmin!(dp[i], dp[i-1] + (h[i] - h[i-1]).abs());
        if i > 1 {
            chmin!(dp[i], dp[i-2] + (h[i] - h[i-2]).abs());
        }
    }
    println!("{}", dp[n-1]);
}
