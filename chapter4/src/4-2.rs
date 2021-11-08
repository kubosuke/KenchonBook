use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: u64
    }
    let mut memo: HashMap<u64, u64> = HashMap::new();
    memo.insert(1,0);
    memo.insert(2, 0);
    memo.insert(3, 1);
    println!("{}", tribo(&mut memo, n));
}

fn tribo(memo: &mut HashMap<u64, u64>, tribo_num: u64) -> u64 {
    println!("called {}", tribo_num);
    match memo.get(&tribo_num).map(|answer| answer.clone()) {
        Some(result) => result,
        None => {
            let result = match tribo_num {
                0 | 1 | 2 => tribo_num,
                n => tribo(memo, n - 1) + tribo(memo, n - 2) + tribo(memo, n - 3),
            };
            memo.insert(tribo_num, result.clone());
            result
        }
    }
}

// 計算量:
// メモ化する場合: O(N)まで抑えられる。
// メモ化しない場合: f(n-1), f(n-2), f(n-3)をそれぞれ独立して求めるので、O(n*n*n) = O(n^3) となる。