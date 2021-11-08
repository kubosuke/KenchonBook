// http://poj.org/problem?id=2456
// https://drken1215.hatenablog.com/entry/2021/06/12/020300

use proconio::input;

const INF: i64 = 1e9 as i64;

// judge whether we can keep each cows away more than 'cost'.
fn ok(v: &Vec<i64>, cows: i64, cost: i64) -> bool {
    let mut pre: i64 = v[0];
    let mut counter: i64 = 0;
    for i in 1..v.len() {
        let barn = v[i];
        if barn - pre >= cost {
            pre = barn;
            counter += 1;
        }
        if counter == cows {
            break
        }
    }
    return counter == cows;
}

fn main() {
    input! {
        n: i64,
        c: i64,
        mut x: [i64; n],
    }
    x.sort();

    let mut left = -1;
    let mut right = INF;

    let mut mid = 0;

    while right - left > 1 {
        mid = (right + left) / 2;
        if ok(&x, c, mid) {
            left = mid + 1
        } else {
            right = mid - 1
        }
    }

    println!("{}", mid)
}
