use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        chars: Chars,
    }
    let str: String = chars.into_iter().collect();
    let ss: Vec<_> = str.split("").collect();
    for s in ss {
        println!("{}", s[2])
    }
}
