// https://atcoder.jp/contests/atc002/tasks/abc007_3
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        h: isize,
        w: isize,
        sy: isize,
        sx: isize,
        gy: isize,
        gx: isize,
        c: [Chars; h],
    }
    let sy= sy - 1;
    let sx= sx - 1;
    let gy= gy - 1;
    let gx= gx - 1;

    let mut dq: VecDeque<(isize, isize, usize)> = VecDeque::new();
    dq.push_front((sy, sx, 0));

    let mut visited = vec![vec![false; w as usize]; h as usize];

    let dyx: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    while !dq.is_empty() {
        let next = dq.pop_back().unwrap();
        if next.0 == gy && next.1 == gx {
            println!("{}", next.2);
            return
        }
        for yx in &dyx {
            let y = next.0 + yx.0;
            let x = next.1 + yx.1;
            if 0 <= y && y < h && 0 <= x && x <= w {
                if c[y as usize][x as usize] == '.' && !visited[y as usize][x as usize] {
                    visited[y as usize][x as usize] = true;
                    dq.push_front((y, x, next.2 + 1));
                }
            }
        }
    }
}
