fn main() {
    let v = vec![3,6,2,154,5,41,564,21,86,9];

    let mut v2 = v.clone();
    heap_sort(&mut v2);
    assert_eq!(v2, vec![2, 3, 5, 6, 9, 21, 41, 86, 154, 564]);
}

// Heapify subtree that have the ith element as root
// target: v[0:n]
fn heapify(v: &mut Vec<i32>, i: usize, n: usize) {
    // child on the left side
    let mut child = i * 2 + 1;

    // if no child, return
    if child >= n {
        return
    }

    // compare children
    if child + 1 < n && v[child + 1] > v[child] {
        child += 1;
    }

    // if no inversion, return
    if v[child] <= v[i] {
        return
    }

    v.swap(i, child);

    heapify(v, child, n)
}

fn heap_sort(v: &mut Vec<i32>) {
    let n = v.len();

    // step 1
    // make v as Heap
    for i in (0..n/2).rev() {
        heapify(v, i, n);
    }

    // step 2
    // pop the maximum value one by one from heap
    for i in (1..n).rev() {
        v.swap(0, i);
        heapify(v, 0, i);
    }
}

