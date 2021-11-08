// refer:
// https://dev.to/jculverhouse/some-number-of-ways-to-calculate-a-fibonacci-number-in-rust-d78?signin=true

use proconio::input;

fn main() {
    input! {
        n: i64,
        a: [i64; n],
        w: i64
    }
    println!("{}", partial_sum(&a, w, 0));
}

// O(N^2)
fn partial_sum(a: &Vec<i64>, w: i64, i: usize) -> bool {
    if i > a.len() - 1 {
        return false
    }
    if w - a[i] == 0 {
        return true
    }
    partial_sum(a, w - a[i], i + 1) || partial_sum(a, w, i + 1)
}
