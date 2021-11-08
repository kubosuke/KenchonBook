// refer:
// https://dev.to/jculverhouse/some-number-of-ways-to-calculate-a-fibonacci-number-in-rust-d78?signin=true

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: u64
    }
    let mut memo: HashMap<u64, u64> = HashMap::new();
    memo.insert(1,1);
    memo.insert(2, 1);
    println!("{}", fibo(&mut memo, n));
}

fn fibo(memo: &mut HashMap<u64, u64>, fib_num: u64) -> u64 {
    println!("called {}", fib_num);
    match memo.get(&fib_num).map(|answer| answer.clone()) {
        Some(result) => result,
        None => {
            let result = match fib_num {
                0 | 1 => fib_num,
                n => fibo(memo, n - 1) + fibo(memo, n - 2),
            };
            memo.insert(fib_num, result.clone());
            result
        }
    }
}
