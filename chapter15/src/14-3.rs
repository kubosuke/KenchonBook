use std::mem::swap;

struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn root(&self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.root(self.par[x])
    }

    fn is_same(&self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x_root = self.root(x);
        let mut y_root = self.root(y);

        if x_root == y_root {
            return false;
        }
        if self.siz[x_root] < self.siz[y_root] {
            swap(&mut x_root, &mut y_root);
        }
        self.par[y_root] = x_root;
        self.siz[x_root] += self.siz[y_root];
        true
    }

    fn size(&self, x: usize) -> usize {
        self.siz[self.root(x)]
    }

    fn is_connected(&self) -> bool {
        for i in 1..self.par.len() {
            if !self.is_same(0, i) {
                return false;
            }
        }
        true
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
    let (n, m) = {
        let v: Vec<usize> = read_vec();
        (v[0], v[1])
    };

    let mut g = {
        let mut gg = vec![];
        for _ in 0..m {
            let (u, v, c) = {
                let v: Vec<usize> = read_vec();
                (v[0], v[1], v[2])
            };
            gg.push((u - 1, v - 1, c));
        }
        gg
    };

    g.sort_by_key(|(_, _, z)| *z);

    let mut uf = UnionFind::new(n);

    let mut mst = vec![];
    let mut mstc = 0;

    for gg in &g {
        if !uf.is_same(gg.0, gg.1) {
            uf.unite(gg.0, gg.1);
            mst.push((gg.0, gg.1, gg.2));
            mstc += gg.2;
        }
    }

    let mut ans = vec![];
    for e in mst {
        let mut uf = UnionFind::new(n);
        let mut c = 0;
        for &gg in &g {
            if gg.0 == e.0 && gg.1 == e.1 {
                continue;
            }
            if !uf.is_same(gg.0, gg.1) {
                uf.unite(gg.0, gg.1);
                c += gg.2;
            }
        }
        if !uf.is_connected() || mstc < c {
            ans.push(e.2);
        }
    }
    println!("{} {}", ans.len(), ans.iter().sum::<usize>());
}
