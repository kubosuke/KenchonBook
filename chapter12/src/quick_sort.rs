fn main() {
    let v = vec![3,6,2,154,5,41,564,21,86,9];

    let mut v2 = v.clone();
    quick_sort(&mut v2, 0, v.len());
    assert_eq!(v2, vec![2, 3, 5, 6, 9, 21, 41, 86, 154, 564]);
}

fn quick_sort(v: &mut Vec<i32>, left: usize, right: usize) {
    // do procedure more than 3 elements exist
    if right - left < 2 {
        return
    }

    // put pivot in the middle
    let pivot_index = (left + right) / 2;
    let pivot = v[pivot_index];

    // swap pivot to the right most side
    v.swap(pivot_index, right-1);

    // seek left -> right
    let mut i = left;
    for j in left..right - 1 {
        if v[j] < pivot {
            // move smaller one to the left side
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, right-1);

    // after that, each elements placed like below:
    // [smaller than pivot(1)][pivot][larger than pivot(2)]
    // do procedure in (1) and (2)
    quick_sort(v, left, i);
    quick_sort(v, i+1, right);
}
