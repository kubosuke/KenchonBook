use proconio::input;

fn main() {
    input! {
        k: i32,
        s: i32
    }
    let mut ans = 0;
    for a in 0..=k {
        for b in 0..=k {
            let z = s - a - b;
                if 0 <= z && z <= k {
                    ans += 1;
                }
            }
        }
    println!("{}", ans)
}
