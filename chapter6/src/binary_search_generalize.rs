// O(log_2(N))
// N: 配列の長さ

fn main() {
    let v: Vec<i32> = vec![2, 5, 8, 10, 14, 17, 21, 39];
    println!("{}", binary_search(&v));
}

// return the minimum index that satisfy p()
fn binary_search(v: &Vec<i32>) -> usize {
    let mut left = 0;
    let mut right = v.len() - 1;
    while right - left > 1 {
        let mid = left + ((right - left) / 2);
        if p(v[mid]) {
            right = mid
        } else {
            left = mid
        }
    }
    right
}

// true if x is greater than 11
fn p(x: i32) -> bool {
    x > 11
}
