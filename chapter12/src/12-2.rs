use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64,
        mut ab: [(i64, i64); n]
    }

    ab.sort_by_key(|&(a, _)|a);

    let mut i = 0;
    let mut ans = 0;
    for _ in 0..m {
        ans += ab[i].0;
        ab[i].1 -= 1;
        if ab[i].1 == 0 {
            i += 1
        }
    }
    println!("{}", ans);
}