use std::fmt;

struct Fenwick {
    tree: Vec<i32>,
}

impl Fenwick {
    /// Constructs a new fenwick tree with the specified `len`
    ///
    /// # Examples
    ///
    /// ```
    /// let f = Fenwick::new(5);
    /// ```
    pub fn new(len: usize) -> Self {
        Self { tree: vec![0; len] }
    }

    /// Update the value at `i` by `w`.
    ///
    /// Complexity: _O_(log_n_).
    pub fn add(&mut self, mut i: usize, w: i32) {
        i += 1;
        while i < self.tree.len() {
            self.tree[i] += w;
            i += lsb(i);
        }
    }

    /// Get a partial sum from 0 to `i`.
    ///
    /// Complexity: _O_(log_n_).
    pub fn sum(&self, mut i: usize) -> i32 {
        let mut res = 0;
        while i != 0 {
            res += self.tree[i];
            i -= lsb(i);
        }
        res
    }
}

impl fmt::Debug for Fenwick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Fenwick")
            .field("tree", &self.tree)
            .finish()
    }
}

/// return least significant bit of i
///
/// # Examples
///
/// ```
/// let x1 = lsb(8);
/// let x1 = lsb(10);
/// let x1 = lsb(24);
/// assert_eq!(8, x1)
/// assert_eq!(2, x2)
/// assert_eq!(8, x3)
/// ```
fn lsb(i: usize) -> usize {
    if i == 0 {
        0
    } else {
        i & !(i - 1)
    }
}

fn main() {
    let v = vec![5,1,2,3,4];
    let mut b = Fenwick::new(v.len()+1);
    let mut ans = 0;
    let mut i = 0;
    while i < v.len() {
        ans += i - b.sum(v[i]) as usize;
        b.add(v[i], 1);
        i += 1;
    }
    // print Fenwick tree
    println!("{:?}", b);

    // print Inversion number
    println!("{}", ans);
}
