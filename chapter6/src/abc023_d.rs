// https://atcoder.jp/contests/abc023/tasks/abc023_d
// ref: https://drken1215.hatenablog.com/entry/2021/06/12/113100

use proconio::input;

const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        b: [(i64, i64); n]
    }
    let mut left = 0;
    let mut right = INF;
    while right - left > 1 {
        let mid = (right + left) / 2;
        let mut ok: bool = true;
        let mut t: Vec<i64> = vec![0; n];

        for i in 0..n {
            if mid < b[i].1 {
                ok = false
            } else {
                t[i] = (mid - b[i].0) / b[i].1
            }
        }
        t.sort();
        for i in 0..n {
            if t[i] < i as i64 {
                ok = false
            }
        }

        if ok {
            right = mid
        } else {
            left = mid
        }
    }
    println!("{}", right)
}
