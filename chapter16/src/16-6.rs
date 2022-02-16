use proconio::input;

pub mod ford_fullkerson {
    use std::{
        cmp::min,
        collections::{HashMap, HashSet},
    };
    pub struct Graph {
        edges: Vec<HashMap<usize, usize>>,
    }

    pub const INF: usize = 18446744073709551615;

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

// S: non break
// T: break

fn main() {
    // Input
    input! {
        n: usize,
        a: [isize; n]
    }

    // Solve
    let s = n;
    let t = n + 1;
    let mut g = ford_fullkerson::Graph::new(n + 2);

    for i in 0..n {
        if a[i] > 0 {
            g.add_edges(s, i, a[i] as usize);
            g.add_edges(i, t, 0);
        } else {
            g.add_edges(i, t, (a[i].abs()) as usize);
        }
    }

    for i in 0..n {
        for j in i + 1..n {
            if (j + 1) % (i + 1) == 0 {
                g.add_edges(j, i, ford_fullkerson::INF);
            }
        }
    }

    // Output
    println!(
        "{}",
        a.iter().filter(|&x| *x > 0).sum::<isize>() - g.max_flow(s, t) as isize
    );
}
