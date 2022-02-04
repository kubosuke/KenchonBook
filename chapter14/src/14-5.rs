use proconio::input;
use std::collections::VecDeque;

const INF: u64 = 18446744073709551615;

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
        k: usize,
    }
    let mut dist = vec![INF; k];
    dist[1] = 1;

    let mut dq = VecDeque::new();
    dq.push_front(1);

    while !dq.is_empty() {
        let v = dq.pop_back().unwrap();

        let v1 = (v * 10) % k;
        if chmin!(dist[v1], dist[v]) {
            dq.push_back(v1);
        }

        let v2 = (v + 1) % k;
        if chmin!(dist[v2], dist[v] + 1) {
            dq.push_front(v2);
        }
    }
    println!("{}", dist[0]);
}
