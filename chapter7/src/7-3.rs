use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(usize,usize); n],
    }
    ab.sort_by_key(|k| k.1);

    let mut cnt = 0;
    for x in ab {
        cnt += x.0;
        if cnt > x.1 {
            println!("No");
            return;
        }
    }
    println!("Yes")
}
