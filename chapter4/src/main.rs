// refer:
// https://dev.to/jculverhouse/some-number-of-ways-to-calculate-a-fibonacci-number-in-rust-d78?signin=true

mod partial_sum;

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: i64,
        a: [i64; n],
    }
    let mut memo: Vec<Vec<Option<bool>>> = vec![vec![None; w as usize + 1]; n + 1];
    println!("{}", partial_sum(&a, w, 0, &mut memo));
}

// O(2^n)
fn partial_sum(a: &Vec<i64>, w: i64, i: usize, memo: &mut Vec<Vec<Option<bool>>>) -> bool {
    if let Some(cache) = memo[i][w as usize] {
        return cache
    }
    if i > a.len() - 1 {
        return false
    }
    if w == a[i] {
        return true
    }

    let c1 = partial_sum(a, w - a[i], i + 1, memo);
    memo[i+1][(w-a[i]) as usize] = Some(c1);
    let c2 = partial_sum(a, w, i + 1, memo);
    memo[i+1][w as usize] = Some(c2);

    c1 || c2
}
