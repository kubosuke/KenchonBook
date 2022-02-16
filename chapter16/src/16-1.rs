// https://atcoder.jp/contests/abc010/tasks/abc010_4

use proconio::input;

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

fn main() {
    input! {
        n:usize, g: usize, e: usize,
        p: [usize; g]
    }
    let mut graph = ford_fullkerson::Graph::new(n + 1);
    for _ in 0..e {
        input! {
            from: usize, to: usize
        }
        graph.add_edges(from, to, 1);
        graph.add_edges(to, from, 1);
    }
    for pp in p {
        graph.add_edges(pp, n, 1);
    }
    println!("{}", graph.max_flow(0, n))
}
