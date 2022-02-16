fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn read_chars() -> Vec<char> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    trim_newline(&mut s);
    s.chars().collect()
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

pub mod ford_fullkerson {
    use std::{
        cmp::min,
        collections::{HashMap, HashSet},
    };
    pub struct Graph {
        edges: Vec<HashMap<usize, usize>>,
    }

    pub const INF: usize = 18446744073709551615 / 100;

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
    let (y, x) = {
        let v: Vec<usize> = read_vec();
        (v[0], v[1])
    };
    // s[y][x]
    let c: Vec<Vec<char>> = {
        let mut tmp: Vec<Vec<char>> = vec![vec![]; y];
        for i in 0..y {
            let mut v: Vec<char> = read_chars();
            tmp[i].append(&mut v);
        }
        tmp
    };
    let mut g = ford_fullkerson::Graph::new(y * x + 2);
    let dxy: Vec<(isize, isize)> = vec![(0, 1), (1, 0)];
    let s = 0;
    let t = (y * x) + 1;

    let mut nume = 0;
    for i in 0..y {
        for j in 0..x {
            for k in 0..2 {
                let nx = j as isize + dxy[k].0;
                let ny = i as isize + dxy[k].1;
                if nx < 0 || nx as usize >= x || ny < 0 || ny as usize >= y {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                let idx = i * x + j + 1;
                let nidx = ny * x + nx + 1;
                if k % 2 == 0 {
                    // up and down
                    if c[i][j] == '#' && c[ny][nx] == '#' {
                        g.add_edges(idx, nidx, ford_fullkerson::INF);
                        g.add_edges(s, idx, 1);
                        nume += 1;
                    }
                } else {
                    // right and left
                    if c[i][j] == '#' && c[ny][nx] == '#' {
                        g.add_edges(idx, nidx, ford_fullkerson::INF);
                        g.add_edges(idx, t, 1);
                        nume += 1;
                    }
                }
            }
        }
    }

    let numh = c.iter().flatten().filter(|&x| *x == '#').count();

    println!("{}", numh - (nume - g.max_flow(s, t)));
}
