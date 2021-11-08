fn binary_search(n: isize, v: &Vec<isize>) -> isize {
    let mut l = 0;
    let mut r = v.len() - 1;

    while r - l >= 0 {
        let mid = (r + l) / 2;
        if v[mid] == n {
            return mid as isize
        } else if v[mid] < n {
            l = mid - 1;
        } else {
            r = mid - 1;
        }
    }
    -1
}

fn main() {
    let ve = vec![3, 5, 8, 10, 14, 17, 21, 39];
    println!("{}", binary_search(10, &ve));
    println!("{}", binary_search(3, &ve));
    println!("{}", binary_search(10, &ve));
    println!("{}", binary_search(3, &ve));
    println!("{}", binary_search(8, &ve));
    println!("{}", binary_search(-100, &ve));
    println!("{}", binary_search(9, &ve));
    println!("{}", binary_search(100, &ve));
}
