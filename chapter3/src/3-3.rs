use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [u32; n]
    }
    let mut b = a;
    b.sort();
    println!("{}", b[1])
}
