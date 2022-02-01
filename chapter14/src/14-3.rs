use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut g = vec![vec![]; n];
    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
        }
        g[u].push(v)
    }

    input! {
        s: Usize1,
        t: Usize1,
    }

    // (v, cost)
    let mut dq = VecDeque::new();
    dq.push_front((s, 0));

    let mut dist: Vec<Vec<isize>> = vec![vec![-1; 3]; n];
    dist[s][0] = 0;

    while !dq.is_empty() {
        let (v, cost) = dq.pop_back().unwrap();
        let p = (cost + 1) % 3;
        for &nv in &g[v] {
            if dist[nv][p] == -1 {
                dq.push_front((nv, cost + 1));
                dist[nv][p] = (cost + 1) as isize;
            }
        }
    }
    println!("{}", {
        if dist[t][0] != -1 {
            dist[t][0] / 3
        } else {
            -1
        }
    });
}
