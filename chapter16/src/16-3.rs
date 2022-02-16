// https://atcoder.jp/contests/abc010/tasks/abc010_4

use proconio::input;
use proconio::marker::Usize1;
use std::collections::{HashSet, VecDeque};
use std::io::{stdout, BufWriter, Write};

pub mod ford_fullkerson {
    use std::cmp;
    const INF: u64 = 18446744073709551615;

    #[derive(Debug, Copy, Clone)]
    pub struct Edge {
        pub from: usize,
        pub to: usize,
        pub cap: u64,
        pub icap: u64,
        pub index: usize,
        pub rev_index: usize,
    }

    pub struct Graph {
        pub list: Vec<Vec<Edge>>,
    }

    impl Graph {
        pub fn new(v_n: usize) -> Self {
            Self {
                list: vec![Vec::new(); v_n],
            }
        }

        fn size(&self) -> usize {
            self.list.len()
        }

        fn edge<'a>(&'a mut self, e: Edge) -> &'a mut Edge {
            &mut self.list[e.from][e.index]
        }

        pub fn redge<'a>(&'a mut self, e: Edge) -> &'a mut Edge {
            &mut self.list[e.to][e.rev_index]
        }

        fn run_flow(&mut self, e: Edge, f: u64) {
            self.edge(e).cap -= f;
            self.redge(e).cap += f;
        }

        pub fn addedge(&mut self, from: usize, to: usize, cap: u64) {
            let last_from_index = self.list[from].len();
            let last_to_index = self.list[to].len();

            self.list[from].push(Edge {
                from,
                to,
                cap,
                icap: cap,
                index: last_from_index,
                rev_index: last_to_index,
            });
            self.list[to].push(Edge {
                from: to,
                to: from,
                cap: 0,
                icap: 0,
                index: last_to_index,
                rev_index: last_from_index,
            });
        }

        fn dfs(&mut self, seen: &mut Vec<bool>, v: usize, t: usize, f: u64) -> Option<u64> {
            if v == t {
                return Some(f);
            }

            seen[v] = true;
            let from_v_edges = self.list[v].clone();

            for e in from_v_edges.into_iter() {
                if seen[e.to] {
                    continue;
                }

                if self.edge(e).cap == 0 {
                    continue;
                }
                let e_cap = self.edge(e).cap;
                match self.dfs(seen, e.to, t, cmp::min(f, e_cap)) {
                    None => {
                        continue;
                    }
                    Some(flow) => {
                        self.run_flow(e, flow);
                        return Some(flow);
                    }
                }
            }

            None
        }
        pub fn max_flow(&mut self, s: usize, t: usize) -> u64 {
            let mut max_flow = 0_u64;
            loop {
                let mut seen = vec![false; self.size()];
                match self.dfs(&mut seen, s, t, INF) {
                    None => {
                        break;
                    }
                    Some(flow) => {
                        max_flow += flow;
                    }
                }
            }
            max_flow
        }
    }
}

fn main() {
    let mut writer = BufWriter::new(stdout());
    loop {
        input! {
            n: usize, m: usize, s: usize, t: usize
        }
        if n + m + s + t == 0 {
            break;
        }
        let s = s - 1;
        let t = t - 1;

        // Ford Fulkerson
        let mut g = ford_fullkerson::Graph::new(n);
        for _ in 0..m {
            input! {
                a: Usize1,
                b: Usize1
            }
            g.addedge(a, b, 1);
        }
        let mf = g.max_flow(s, t);

        // Search traversable vertices from s, t respectively
        //
        // v_i => v_j
        // ^^^^^^^^^^ S
        //
        let mut sq = VecDeque::new();
        let mut S = HashSet::new();
        sq.push_front(s);
        while !sq.is_empty() {
            let v = sq.pop_back().unwrap();
            S.insert(v);
            for e in &g.list[v] {
                if !S.contains(&e.to) && e.cap > 0 {
                    sq.push_front(e.to);
                }
            }
        }
        //
        // v_i => v_j
        // ^^^^^^^^^^ T
        //
        let mut tq = VecDeque::new();
        let mut T = HashSet::new();
        tq.push_front(t);
        while !tq.is_empty() {
            let v = tq.pop_back().unwrap();
            T.insert(v);
            for e in &g.list[v].clone() {
                // caution: redge!!
                if !T.contains(&e.to) && &g.redge(*e).cap > &0 {
                    tq.push_front(e.to);
                }
            }
        }

        // Find T -> S path whose cap is larger than 0
        //
        // v_i => v_j
        // ^^^ T  ^^^ S
        //
        let mut cnt = 0;
        for &v in T.iter() {
            for e in &g.list[v] {
                if S.contains(&e.to) && e.cap > 0 && e.cap == e.icap {
                    cnt += 1;
                }
            }
        }

        let nmf = {
            if cnt > 0 {
                mf + 1
            } else {
                mf
            }
        };
        writeln!(writer, "{} {}", nmf, cnt).ok();
    }
    writer.flush().unwrap();
}
