use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

const INF: usize = 4;

macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_min = min!($($cmps),+);
        if $base > cmp_min {
            $base = cmp_min;
            true
        } else {
            false
        }
    }};
}

macro_rules! min {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::min($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::min($a, min!($($rest),+))
    }};
}

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut sx = 0;
    let mut sy = 0;
    let mut gx = 0;
    let mut gy = 0;
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == 's' {
                sx = i;
                sy = j;
            }
            if c[i][j] == 'g' {
                gx = i;
                gy = j;
            }
        }
    }

    let mut dq = VecDeque::new();
    dq.push_front((sx, sy, 0));

    let dxy: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];
    let mut bc = vec![vec![INF; w]; h];
    bc[sx][sy] = 0;

    while !dq.is_empty() {
        let v = dq.pop_back().unwrap();
        if v.0 == gx && v.1 == gy {
            println!("YES");
            return;
        }
        for &d in &dxy {
            let dx = v.0 as isize + d.0;
            let dy = v.1 as isize + d.1;
            if dx < 0 || dx > (h - 1) as isize {
                continue;
            }
            if dy < 0 || dy > (w - 1) as isize {
                continue;
            }
            if c[dx as usize][dy as usize] == '#' {
                if v.2 > 1 {
                    continue;
                }
                if !chmin!(bc[dx as usize][dy as usize], v.2 + 1) {
                    continue;
                }
                dq.push_front((dx as usize, dy as usize, v.2 + 1));
            } else {
                if !chmin!(bc[dx as usize][dy as usize], v.2) {
                    continue;
                }
                dq.push_front((dx as usize, dy as usize, v.2));
            }
        }
    }
    println!("NO")
}
