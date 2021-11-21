use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n]
    }
    a.sort();
    b.sort();

    let mut i = 0;
    for j in 0..n {
        if a[i] < b[j] {
            i += 1;
        }
    }
    println!("{}", i)
}
