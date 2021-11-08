use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }
    let mut dp: Vec<i32> = vec![0; n];
    dp[0] = 0;
    dp[1] = (h[1] - h[0]).abs();
    for i in 2usize..=n-1 {
        dp[i] = std::cmp::min((h[i] - h[i-1]).abs() + dp[i-1], (h[i] - h[i-2]).abs() + dp[i-2]);
    }
    println!("{}", dp[n-1]);
}
