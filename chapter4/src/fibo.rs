use proconio::input;

fn main() {
    input! {
        n: i32
    }
    println!("{}", fibo(n));
}

fn fibo(n: i32) -> i32 {
    if n == 1 || n == 2 {
        return 1
    }
    return fibo(n-2) + fibo(n-1)
}