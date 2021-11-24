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
            par:(0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn root(&self, x: usize) -> usize {
        if self.par[x] == x {
            return x
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
            return false
        }
        if self.siz[x_root] < self.siz[y_root] {
            swap(&mut x_root, &mut y_root);
        }
        self.par[y_root]= x_root;
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

    let mut cnt = 0;

    // i: candidates
    for i in 0..m {
        let mut uf =  UnionFind::new(n);
        for j in 0..m {
            if i == j {
                continue
            }
            // edges are 1-indexed
            uf.unite(edges[j].0-1, edges[j].1-1);
        }
        let flag = uf.root(0);
        for nn in 1..n {
            if uf.root(nn) != flag {
                cnt += 1;
                break
            }
        }
    }

    println!("{}", cnt);
}
