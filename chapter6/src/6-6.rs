// https://atcoder.jp/contests/arc037/tasks/arc037_c

use proconio::input;
use std::f64::consts::PI;

fn f(abc: (f64, f64, f64), t: f64) -> f64 {
    (abc.0*t) + abc.1 * (PI * abc.2 * t).sin()
}

fn main() {
    input! {
        abc: (f64,f64,f64)
    }
    let mut right = 200.0;
    let mut left = 0.0;
    while right - left > 0.0000000001 {
        let mid = (right + left) / 2.0;
        if f(abc, mid) <= 100.0 {
            left = mid
        } else {
            right = mid
        }
    }
    println!("{}", left)
}
