use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [u32; n]
    }
    let (ma,mi) = (a.iter().max().unwrap(), a.iter().min().unwrap());
    println!("{}", ma-mi)
}
