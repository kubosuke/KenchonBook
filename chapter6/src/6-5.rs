// https://atcoder.jp/contests/arc037/tasks/arc037_c

use proconio::input;

const INF: i64 = 1<<60;

fn check(a: &Vec<i64>, b: &Vec<i64>, x: i64, k: usize) -> bool {
    let mut cnt = 0;
    let mut j = 0;
    for a in a.iter().rev() {
        while j < b.len() && *a * b[j] <= x {
            j += 1;
        }
        cnt += j;
    }
    return cnt >= k
}

fn main() {
    input! {
        n: i64,
        k: i64,
        mut a: [i64; n],
        mut b: [i64; n],
    }
    a.sort();
    b.sort();

    let mut left = 0;
    let mut right = INF;

    // 'k' th minimum value
    // => minimum x | the number of value that satisfy < x is K in []a*b
    // in this case, x is the upper bound of []a*b which satisfy 'k' th minimum value
    while right - left > 1 {
        let mid = (left + right) / 2;
        if check(&a, &b, mid, k as usize) {
            right = mid
        } else {
            left = mid
        }
    }
    println!("{}", right)
}
