// https://drken1215.hatenablog.com/entry/2019/06/17/221400

use proconio::input;
use proconio::marker::Chars;

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
        r: isize,c: isize,
        m: [Chars; r],
    }

    let dx: Vec<isize> = vec![1, 0, -1, 0];
    let dy: Vec<isize> = vec![0, 1, 0, -1];

    // init
    // g[y][x]
    let mut g = ford_fullkerson::Graph::new((r * c) as usize + 2);

    // edge from/to super nodes
    for y in 0..r {
        for x in 0..c {
            let n = (x + (y * c) + 1) as usize;
            if (x + y) % 2 == 0 {
                g.add_edges(0, n, 1);
            } else {
                g.add_edges(n, (r * c + 1) as usize, 1);
            }
        }
    }

    // edge between nodes
    for y in 0..r {
        for x in 0..c {
            for i in 0..4 {
                let nx = x + dx[i];
                let ny = y + dy[i];
                if nx < 0 || nx >= c || ny < 0 || ny >= r {
                    continue;
                }
                if m[y as usize][x as usize] == '.' && m[ny as usize][nx as usize] == '.' {
                    let n = (x + (y * c) + 1) as usize;
                    let nn = (nx + (ny * c) + 1) as usize;
                    if (x + y) % 2 == 0 {
                        g.add_edges(n, nn, 1);
                    } else {
                        g.add_edges(nn, n, 1);
                    }
                }
            }
        }
    }

    // count .
    let cnt = m.iter().flatten().filter(|&c| *c == '.').count();

    // ans
    println!("{}", cnt - g.max_flow(0, (r * c + 1) as usize));
}
