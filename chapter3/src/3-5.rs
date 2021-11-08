use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [u32; n]
    }
    let mut ans: i32 = 20000;
    for x in a {
        let mut c = 0;
        let mut b = x;
        while b %2 == 0 {
            b /= 2;
            c += 1;
        }
        ans = std::cmp::min(ans, c)
    }
    println!("{}", ans)
}
