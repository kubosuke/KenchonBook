fn main() {
    let v: Vec<i32> = vec![3, 5, 8, 10, 14, 17, 21, 39];
    println!("{}", binary_search(3, &v));
    println!("{}", binary_search(17, &v));
    println!("{}", binary_search(100, &v));
}

fn binary_search(n: i32, v: &Vec<i32>) -> isize {
    let mut left = 0;
    let mut right = v.len() - 1;
    while left <= right {
        let mid = left + ((right - left) / 2);
        if v[mid] == n {
            return mid as isize
        } else if v[mid] > n {
            right = mid-1;
        } else {
            left = mid+1;
        }
    }
    -1
}
