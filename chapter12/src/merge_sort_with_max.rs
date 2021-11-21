fn main() {
    let v = vec![3,6,2,154,5,41,564,21,86,9];

    let mut v2 = v.clone();
    merge_sort(&mut v2, 0, v.len());
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

fn merge(v: &mut Vec<i32>, left: usize, mid: usize, right: usize) {
    let mut v_left = v[left..=mid].to_vec();
    let mut v_right = v[mid+1..right].to_vec();

    for i in left.. {
        let v_left_min = v_left.iter().min();
        let v_right_min = v_right.iter().min();
        if v_left_min.is_some() && v_right_min.is_some() {
            let v_left_min_index = v_left.iter().position(|e| e == v_left_min.unwrap()).unwrap();
            let v_right_min_index = v_right.iter().position(|e| e == v_right_min.unwrap()).unwrap();
            if *v_left_min.unwrap() < *v_right_min.unwrap() {
                v[i] = *v_left_min.unwrap();
                v_left.remove(v_left_min_index);
            } else {
                v[i] = *v_right_min.unwrap();
                v_right.remove(v_right_min_index);
            }
            continue
        }
        if v_left_min.is_some() {
            let v_left_min_index = v_left.iter().position(|e| e == v_left_min.unwrap()).unwrap();
            v[i] = *v_left_min.unwrap();
            v_left.remove(v_left_min_index);
            continue
        }
        if v_right_min.is_some() {
            let v_right_min_index = v_right.iter().position(|e| e == v_right_min.unwrap()).unwrap();
            v[i] = *v_right_min.unwrap();
            v_right.remove(v_right_min_index);
            continue
        }
        break
    }
}
