fn main() {
    let v = vec![3,6,2,154,5,41,564,21,86,9];

    let mut v2 = v.clone();
    merge_sort(&mut v2, 0, v.len()-1);
    assert_eq!(v2, vec![2, 3, 5, 6, 9, 21, 41, 86, 154, 564]);
}

// left, right: 0-index of v
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
        match (left_itr.peek(), right_itr.peek()) {
            (None, Some(_)) => v[i] = *right_itr.next().unwrap(),
            (Some(_), None) => v[i] = *left_itr.next().unwrap(),
            (Some(l), Some(r)) if l < r => v[i] = *left_itr.next().unwrap(),
            (Some(l), Some(r)) if r < l => v[i] = *right_itr.next().unwrap(),
            (Some(_), Some(_)) => v[i] = *left_itr.next().unwrap(),
            _ => break
        }
    }
}
