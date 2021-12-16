// https://agw.hatenablog.jp/entry/20090513/1242290109
fn main() {
    let v = vec![1, 4, 8, 3, 99, 42, 67];
    let mid = select_kth( v.clone(), 3);
    println!("{}", mid)
}

// selects the kth smallest element
fn select_kth(mut a: Vec<i32>, k: usize) -> i32 {
    // 1. if length of a is less than 100, return kth smallest value
    if a.len() <= 100 {
        a.sort();
        return a[k]
    };

    // 2. find a good pivot
    // split every 5 elements and find mid of each chunks
    //
    //  mid1                           mid2
    //  |                               |
    //  V                               V
    // |8, 5, 12, 81, 3|18, 36, 12, 7, 14|...
    //
    let mut a2: Vec<i32> = vec![];
    for i in (0..a.len()).into_iter().step_by(5) {
        let mut a3 = if i + 5 > a.len() {
            a[i..].to_vec()
        } else {
            a[i..i+5].to_vec()
        };
        a3.sort();
        a2.push(a3[2]);
    };


    // select mid of a2
    // second args is a.len()/10: 'cause a.len() = a2.len() * 5
    let mid = select_kth(a2, a.len()/10);

    // after you find mid, let's divide among 3 parts:
    // p: less than mid
    // q: equals mid
    // r: more than mid
    let mut p: Vec<i32> = vec![];
    let mut q: Vec<i32> = vec![];
    let mut r: Vec<i32> = vec![];

    for i in 0..a.len() {
        if a[i] < mid {
            p.push(a[i]);
        } else if a[i] == mid {
            q.push(a[i]);
        } else {
            r.push(a[i]);
        }
    };

    if k <= p.len() {
        return select_kth(p, k);
    } else if k <= p.len() + q.len() {
        return mid;
    } else {
        return select_kth(r, k - p.len() - q.len())
    };
}
