// https://atcoder.jp/contests/abc061/tasks/abc061_d

use proconio::input;
use std::collections::HashSet;

const INF: i64 = 1_000_000_000_000;

macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_min = min!($($cmps),+);
        if $base > cmp_min {
            $base = cmp_min;
            true
        } else {
            false
        }
    }};
}

macro_rules! min {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::min($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::min($a, min!($($rest),+))
    }};
}

fn main() {
    input! {
        n: usize, // 1-index
        m: usize, // 1-index
        abc: [(usize, usize, i64); m] // 1-index
    }
    let graph: Vec<HashSet<(usize, i64)>> = {
        let mut hs: Vec<HashSet<(usize, i64)>> = vec![HashSet::new(); n];
        for x in abc {
            hs[x.0 - 1].insert((x.1 - 1, -x.2)); // 0-index
        }
        hs
    };

    let mut dist = vec![INF; n];
    dist[0] = 0;
    for i in 0..=n * 2 {
        for j in 0..n {
            if dist[j] == INF {
                continue;
            }
            for g in &graph[j] {
                if chmin!(dist[g.0], dist[j] + g.1) {
                    // goalを含む負閉路をfiltering
                    if i >= n - 1 && g.0 == n - 1 {
                        println!("inf");
                        return;
                    }
                }
            }
        }
    }
    println!("{}", -dist[n - 1]);
}
