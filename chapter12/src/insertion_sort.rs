fn main() {
    let v = vec![3,6,2,154,3,41,564,21,86,9];
    assert_eq!(insertion_sort(v), vec![2, 3, 3, 6, 9, 21, 41, 86, 154, 564]);
}

fn insertion_sort(v: Vec<i32>) -> Vec<i32> {
    let mut v = v;
    for i in 0..v.len() {
        for j in (0..i).rev() {
            if v[j] > v[j+1] {
                v.swap(j, j+1);
            }
        }
    }
    v
}
