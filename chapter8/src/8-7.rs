use std::collections::HashMap;

const K: i32 = 15;

fn main() {

    let a = vec![1, 2, 4, 8, 10];
    let b = vec![3, 5, 7, 9, 11];

    let mut hash_map = HashMap::new();

    // O(2N) = O(N)
    for x in a {
        if x > K {
            continue
        }
        hash_map.insert(K-x, true);
    }
    for y in b {
        match hash_map.get(&y) {
            Some(_) => {
                println!("Yes");
                return;
            },
            _ => ()
        }
    }
    println!("No");
}
