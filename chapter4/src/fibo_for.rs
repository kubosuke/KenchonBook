use proconio::input;

fn main() {
    input! {
        n: i32
    }
    println!("{}", fibo(n));
}

fn fibo(n: i32) -> i32 {
    let mut v = vec![1,1];
    for i in 2..=n {
        v.push(v[i as usize -2] + v[i as usize -1]);
    }
    return v[n as usize - 1];
}
