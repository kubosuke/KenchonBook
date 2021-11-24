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
    let mut uf = UnionFind::new(7);

    uf.unite(1, 2);
    uf.unite(2, 3);
    uf.unite(5, 6);

    assert_eq!(true, uf.is_same(1, 3));
    assert_eq!(false, uf.is_same(2, 5));
    assert_eq!(3, uf.size(1));
    uf.unite(1, 6);
    assert_eq!(true, uf.is_same(2, 5));
    assert_eq!(5, uf.size(1));
}
