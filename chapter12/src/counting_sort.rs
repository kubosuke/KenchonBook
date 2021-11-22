// on the assumption that each elements is less than 100000
const MAX: usize = 100000;

fn main() {
    let mut v = vec![3,6,2,154,5,41,564,21,86,9];
    assert_eq!(counting_sort(&mut v), vec![2, 3, 5, 6, 9, 21, 41, 86, 154, 564]);
}

fn counting_sort(v: &mut Vec<i32>) -> Vec<i32> {
    let n = v.len();

    // count each elements
    let mut num = vec![0; MAX];
    for i in 0..n {
        num[v[i] as usize] += 1
    }

    // get cumulative sum of num
    let mut sum = vec![0; MAX];
    for value in 1..MAX {
        sum[value] = sum[value-1] + num[value];
    }

    let mut v2: Vec<i32> = vec![0; n];
    for i in (0..n).rev() {
        v2[sum[v[i] as usize] - 1] = v[i];
    }
    v2
}
