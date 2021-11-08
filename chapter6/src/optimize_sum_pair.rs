struct BinarySearch<F> {
    p: F,
    left: i64,
    right: i64,
}

impl <F: Fn(i64) -> bool> BinarySearch<F> {
    fn lower_bound(&self) -> i64 {
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
            right
        } else {
            -1
        }
    }
}

// note:
// a + b >= k
// find minimum b which satisfy b >= k - a

fn main() {
    let a: Vec<i64> = vec![2, 5, 8, 10, 14, 17, 21, 39];
    let b: Vec<i64> = vec![1, 3, 6, 12, 15, 19, 23, 41];
    let k = 56;

    for i in 0..a.len() {
        let bs = BinarySearch {
            p: |x| {
                b[x as usize] >= k - a[i]
            },
            left: 0,
            right: (b.len()-1) as i64,
        };
        let index = bs.lower_bound();
        if index != -1 {
            println!("{}", a[i] + b[index as usize])
        } else {
            println!("{}", -1)
        }
    }
}
