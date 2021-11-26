use std::collections::HashMap;

fn main() {
    let a = vec![1, 3, 5, 5, 8, 20, 5, 50, 89, 136];
    let b = vec![4, 5, 23, 40, 18, 50, 89, 91, 100, 120];

    let mut hash_map = HashMap::new();

    // O(N + M)
    let mut tmp = 0;
    for x in [a, b].concat() {
        let cnt = hash_map.entry(x).or_insert(0);
        if *cnt == 0 {
            tmp += 1;
        }
        *cnt += 1;
    }

    assert_eq!(5, hash_map.values().sum::<i32>() - tmp);
}
