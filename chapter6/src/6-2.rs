use proconio::input;

struct BinarySearch<F> {
    p: F,
    left: isize,
    right: isize,
}

impl <F: Fn(isize) -> bool> BinarySearch<F> {
    fn lower_bound(&self) -> isize {
        let mut left = self.left;
        let mut right = self.right;

        while right - left > 1 {
            let mid = (left+right)/2;
            if (self.p)(mid) {
                right = mid;
            } else {
                left = mid;
            }
        }
        if (self.p)(right) {
            // if all nodes satisfy condition p, return 0
            if (self.p)(left) {
                return 0
            }
            right
        } else {
            -1
        }
    }
}

fn main() {
    input! {
        n: isize,
        mut a: [isize; n],
        mut b: [isize; n],
        mut c: [isize; n],
    }
    let mut ans: isize = 0;
    a.sort_by(|a, b| b.cmp(a));
    b.sort();
    c.sort();
    for bb in b {
        let bsa = BinarySearch{
            p: |x| {
                a[x as usize] < bb
            },
            left: 0,
            right: n-1
        };
        let bsc = BinarySearch{
            p: |x| {
                c[x as usize] > bb
            },
            left: 0,
            right: n-1
        };
        let ai = bsa.lower_bound();
        let ci = bsc.lower_bound();
        if ai != -1 && ci != -1 {
            ans += (n - ai) * (n - ci);
        }
    }
    println!("{}", ans)
}
