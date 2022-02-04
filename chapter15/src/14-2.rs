use proconio::input;
use proconio::marker::Usize1;
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
}

fn main() {
    let mut ans = vec![];
    loop {
        input! {
            n: usize,
            m: usize,
        }
        if n == 0 && m == 0 {
            break;
        }
        let mut g = {
            let mut gg = vec![];
            for _ in 0..m {
                input! {
                    u: Usize1,
                    v: Usize1,
                    c: usize,
                }
                gg.push((u, v, c));
            }
            gg
        };

        // kruskal
        g.sort_by_key(|(_, _, z)| *z);

        let mut uf = UnionFind::new(n);
        let mut c = vec![];

        for gg in g {
            if !uf.is_same(gg.0, gg.1) {
                uf.unite(gg.0, gg.1);
                c.push(gg.2);
            }
        }

        c.sort();
        ans.push(c[c.len() / 2]);
    }
    for a in ans {
        println!("{}", a);
    }
}
