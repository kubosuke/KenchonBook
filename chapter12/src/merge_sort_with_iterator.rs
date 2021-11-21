fn main() {
    let v = vec![3,6,2,154,5,41,564,21,86,9];

    let mut v2 = v.clone();
    merge_sort(&mut v2, 0, v.len()-1);
    assert_eq!(v2, vec![2, 3, 5, 6, 9, 21, 41, 86, 154, 564]);
}

fn merge_sort(v: &mut Vec<i32>, left: usize, right: usize) {
    if right > left {
        let mid = (left + right)/2;
        merge_sort(v, left, mid);
        merge_sort(v, mid+1, right);
        merge(v, left, mid, right);
    }
}

fn merge(v: &mut Vec<i32>, left: usize, mid:usize, right:usize) {
    let left_vec = v[left..=mid].to_vec();
    let right_vec = v[mid+1..=right].to_vec();

    let mut left_itr = left_vec.iter().peekable();
    let mut right_itr = right_vec.iter().peekable();

    for i in left..=right {
        if left_itr.peek().is_none() && right_itr.peek().is_none() {
            break
        }
        if left_itr.peek().is_none() {
            v[i] = *right_itr.next().unwrap();
            continue
        }
        if right_itr.peek().is_none() {
            v[i] = *left_itr.next().unwrap();
            continue
        }
        if *left_itr.peek().unwrap() < *right_itr.peek().unwrap() {
            v[i] = *left_itr.next().unwrap();
        } else {
            v[i] = *right_itr.next().unwrap();
        }
    }
}
