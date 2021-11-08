// https://atcoder.jp/contests/arc101/tasks/arc101_b
// https://drken1215.hatenablog.com/entry/2018/09/08/011500

use proconio::input;

// nearly exceeds 10^9
const A_MAX: i64 = 1<<30;

fn main() {
    input! {
        n: i64,
        mut a: [i64; n]
    }

    if n == 1 {
        println!("{}", a[0]);
        return
    }

    let mut left = 0;
    let mut right = A_MAX;

    while right - left > 1 {
        let x = (right + left) / 2;

        // preprocessing:
        // convert 1 if the value is above x
        // convert -1 if the value is less than x
        let aa: Vec<i64> = a.iter().map(|&ai| c(ai, x)).collect();

        // calculate cumulative sums of aa
        // FYI about scan(): https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan
        let mut c_sums: Vec<i64> = aa
            .iter()
            .scan(0, |aaa, x| {
               *aaa += x;
                Some(*aaa)
            })
            .collect();
        c_sums.insert(0, 0);

        let mut flag = 0;
        for l in 0..n as usize {
            for r in l..n as usize {
                if c_sums[r] >= c_sums[l] {
                    flag += 1
                }
            }
        }

        if flag > (combination(n + 1, 2) / 2) + 1 {
            left = x;
        } else {
            right = x;
        }
    }
    println!("{}", left);
}

fn c(ai: i64, x: i64) -> i64 {
    if ai >= x {
        return 1;
    }
    return -1;
}

fn factorial(x: i64) -> i64 {
    (1..=x).product()
}

fn combination(n: i64, r:i64) -> i64 {
    factorial(n) / (factorial(n-r) * factorial(r))
}
