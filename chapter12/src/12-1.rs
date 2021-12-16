fn main() {
    let n = 10;
    let v = vec![3,6,2,154,5,41,564,21,86,9];

    let mut tv: Vec<(usize, i32)> = vec![];
    for i in 0..n {
        tv.push((i, v[i]))
    }

    tv.sort_by_key(|&(_, k)|k);

    for i in 0..n {
        println!("i = {}, ai = {}, order = {}", tv[i].0, tv[i].1, i+1);
    }
}

// i = 2, ai = 2, order = 1
// i = 0, ai = 3, order = 2
// i = 4, ai = 5, order = 3
// i = 1, ai = 6, order = 4
// i = 9, ai = 9, order = 5
// i = 7, ai = 21, order = 6
// i = 5, ai = 41, order = 7
// i = 8, ai = 86, order = 8
// i = 3, ai = 154, order = 9
// i = 6, ai = 564, order = 10

