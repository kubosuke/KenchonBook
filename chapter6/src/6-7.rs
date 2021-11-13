// https://atcoder.jp/contests/arc101/tasks/arc101_b

use proconio::input;

// nearly exceeds 10^9
const A_MAX: i64 = 1<<30;

struct Fenwick {
    tree: Vec<i64>,
}

impl Fenwick {
    /// create Fenwick Tree by given length
    pub fn new(len: usize) -> Self {
        Self { tree: vec![0; len] }
    }

    /// add `w` to index `i`
    /// `i` is 1-indexed
    pub fn add(&mut self, mut i: usize, w: i64) {
        while i < self.tree.len() {
            self.tree[i] += w;
            i += lsb(i);
        }
    }

    /// sum `[1, a]`
    /// `a` is 1-indexed
    pub fn sum(&self, mut i: usize) -> i64 {
        let mut res = 0;
        while i > 0 {
            res += self.tree[i];
            i -= lsb(i);
        }
        res
    }

    /// sum `[a, b)`
    /// a and b are 1-indexed
    pub fn partial_sum(&self, a: usize, b: usize) -> i64 {
        self.sum(b-1) - self.sum(a-1)
    }
}

/// return least significant bit of i
fn lsb(i: usize) -> usize {
    if i == 0 {
        0
    } else {
        i & !(i - 1)
    }
}

fn main() {
    input! {
        n: i64,
        mut a: [i64; n]
    }

    if n == 1 {
        println!("{}", a[0]);
        return
    }

    let mut left = 0;
    let mut right = A_MAX;
    let geta = n + 1;
    while right - left > 1 {
        let mid = (right + left) / 2;
        let mut inversion_number = 0;
        let mut ft = Fenwick::new(n as usize * 2 + 10);
        let mut sum = 0;
        ft.add((sum + geta) as usize, 1);
        for i in 0..n as usize {
            let val = if a[i] <= mid { 1 } else { -1 };
            sum += val;
            inversion_number += ft.partial_sum(1, (sum+geta) as usize);
            ft.add((sum+geta) as usize, 1);
        }

        if inversion_number as i64 > (n+1)*n/2/2 {
            right = mid;
        } else {
            left = mid;
        }
    }
    println!("{}", right);
}
