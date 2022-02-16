// use proconio::input;
use std::io::{stdout, BufWriter, Write};

pub mod ford_fullkerson {
    use std::{
        cmp::min,
        collections::{HashMap, HashSet},
    };
    pub struct Graph {
        edges: Vec<HashMap<usize, usize>>,
    }

    const INF: usize = 18446744073709551615;

    impl Graph {
        pub fn new(n: usize) -> Self {
            Graph {
                edges: vec![HashMap::new(); n],
            }
        }
        pub fn add_edges(&mut self, from: usize, to: usize, capacity: usize) {
            *self.edges[from].entry(to).or_insert(0) += capacity;
            self.edges[to].entry(from).or_insert(0);
        }
        fn run_flow(&mut self, from: usize, to: usize, flow: usize) {
            *self.edges[from].entry(to).or_insert(0) -= flow;
            *self.edges[to].entry(from).or_insert(0) += flow;
        }
        pub fn max_flow(&mut self, start: usize, goal: usize) -> usize {
            let mut flow = 0;
            loop {
                let f = self.dfs(start, goal, INF, &mut HashSet::new());
                if f == 0 {
                    break;
                } else {
                    flow += f;
                }
            }
            flow
        }
        pub fn dfs(
            &mut self,
            now: usize,
            goal: usize,
            flow: usize,
            visited: &mut HashSet<usize>,
        ) -> usize {
            if now == goal {
                return flow;
            }
            visited.insert(now);
            for (dest, capacity) in self.edges[now].clone() {
                if !visited.contains(&dest) && capacity > 0 {
                    let f = self.dfs(dest, goal, min(flow, capacity), visited);
                    if f > 0 {
                        self.run_flow(now, dest, f);
                        return f;
                    }
                }
            }
            0
        }
    }
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn main() {
    let mut writer = BufWriter::new(stdout());
    loop {
        // input! {
        //     m: usize, n: usize
        // }
        let (m, n) = {
            let v: Vec<usize> = read_vec();
            (v[0], v[1])
        };
        if m + n == 0 {
            break;
        }
        // input! {
        //     b: [usize; m],
        //     r: [usize; n],
        // }
        let b = {
            let mut tmp = vec![];
            for _ in 0..=(m - 1) / 10 {
                let mut v = read_vec();
                tmp.append(&mut v);
            }
            tmp
        };
        let r = {
            let mut tmp = vec![];
            for _ in 0..=(n - 1) / 10 {
                let mut v = read_vec();
                tmp.append(&mut v);
            }
            tmp
        };
        let mut g = ford_fullkerson::Graph::new(n + m + 2);
        for i in 0..m {
            for j in 0..n {
                if gcd(b[i], r[j]) > 1 {
                    g.add_edges(i + 1, j + 1 + m, 1);
                }
            }
        }
        for v in 1..=m {
            g.add_edges(0, v, 1);
        }
        for v in 1 + m..=n + m {
            g.add_edges(v, n + m + 1, 1);
        }
        writeln!(writer, "{}", g.max_flow(0, n + m + 1)).unwrap();
    }
    writer.flush().unwrap();
}

fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
