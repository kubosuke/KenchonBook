use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32
    }
    println!("{}", euclid(a, b));
}

fn euclid(a: i32, b: i32) -> i32 {
    let r = a%b;
    if r == 0 {
        return b
    }
    return euclid(b, r)
}