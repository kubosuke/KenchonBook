#![allow(dead_code)]

use proconio::input;
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
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }

    let mut uf = UnionFind::new(n);
    let mut c = n * (n - 1) / 2;
    let mut v = vec![];

    // seek from end:
    // union find is hard to manage delete,
    // think from a different angle that "add from nothing".
    for i in (0..m).rev() {
        v.push(c);
        let a = edges[i].0 - 1;
        let b = edges[i].1 - 1;
        if uf.root(a) == uf.root(b) {
            continue;
        }
        c -= uf.size(a) * uf.size(b);
        uf.unite(a, b);
    }
    for i in (0..m).rev() {
        println!("{}", v[i])
    }
}
