//ref: https://drken1215.hatenablog.com/entry/2019/04/03/125400

use proconio::input;

fn main() {
    input! {
        n: u32,
    }
    let mut c = 0;
    solve(n, 0, 0b000, &mut c);
    println!("{}", c)
}

fn solve(n: u32, current: u32, flags: u8, counter: &mut u32) {
    if current > n {
        return
    }
    if flags == 0b111 {
        *counter += 1
    }
    solve(n, current*10 + 3, flags | 0b001, counter);
    solve(n, current*10 + 5, flags | 0b010, counter);
    solve(n, current*10 + 7, flags | 0b100, counter);
}
