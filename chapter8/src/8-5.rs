use std::collections::HashMap;

// O(N+M)
fn main() {
    let a = vec![1, 3, 5, 6, 8, 20, 30, 50, 89, 136];
    let b = vec![4, 5, 23, 40, 18, 50, 89, 91, 100, 120];

    let mut hash_map = HashMap::new();

    // O(N)
    for aa in a {
        hash_map.add(aa, true);
    }

    // O(M)
    let mut ans = 0;
    for bb in b {
        if hash_map.contains_key(&bb) {
            ans += 1
        }
    }

    assert_eq!(3, ans);
}
