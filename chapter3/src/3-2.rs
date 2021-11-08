use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [u32; n],
        v: u32
    }
    println!("{}",  a.iter().filter(|&&i| i == v).count())
}
