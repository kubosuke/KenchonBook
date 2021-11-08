// https://www.ioi-jp.org/joi/2007/2008-ho-prob_and_sol/2008-ho.pdf#page=6

// 問題
// あなたは以下のルールでダーツゲームをすることになった．
// あなたは，矢を的（まと）に向かって 4 本まで投げることができる．必ず
// しも 4 本全てを投げる必要はなく，1 本も投げなくてもかまわない．的
// は N 個の部分に区切られていて，各々の部分に点数 P1, · · · , PN が書か
// れている．矢が刺さった場所の点数の合計 S があなたの得点の基礎とな
// る．S があらかじめ決められたある点数 M 以下の場合は S がそのまま
// あなたの得点となる．しかし，S が M を超えた場合はあなたの得点は
// 0 点となる．
// 的に書かれている点数と M の値が与えられたとき，あなたが得ることのできる
// 点数の最大値を求めるプログラムを作成せよ．

// 入力
// 入力ファイルのファイル名は input.txt である．
// 1 行目には，整数 N(1 5 N 5 1000) と M(1 5 M 5 200000000 = 2 × 108
// ) がこ
// の順に空白区切りで書かれている．2 行目以降の第 1 + i 行目 (1 5 i 5 N) には，
// Pi(1 5 Pi 5 100000000 = 108
// ) が書かれている．
// 採点用データのうち, 配点の 20% 分は N 5 100 を満たし，配点の 50% 分は
// N 5 300 を満たす．

use proconio::input;

#[allow(unused_macros)]
macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_max = max!($($cmps),+);
        if $base < cmp_max {
            $base = cmp_max;
            true
        } else {
            false
        }
    }};
}

#[allow(unused_macros)]
macro_rules! max {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::max($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::max($a, max!($($rest),+))
    }};
}

struct BinarySearch<F> {
    p: F,
    left: isize,
    right: isize,
}

impl <F: Fn(isize) -> bool> BinarySearch<F> {
    fn lower_bound(&self) -> isize {
        let mut left = self.left;
        let mut right = self.right;

        while right - left > 1 {
            let mid = (left+right)/2;
            if (self.p)(mid) {
                right = mid;
            } else {
                left = mid;
            }
        }
        if (self.p)(right) {
            // if all nodes satisfy condition p, return 0
            if (self.p)(left) {
                return 0
            }
            right
        } else {
            -1
        }
    }
}

fn main() {
    input! {
        n: isize,
        m: isize,
        mut a: [isize; n],
    }
    let mut ans: isize = 0;

    a.push(0);

    let mut b: Vec<isize> = vec![];
    for i in 0..=n as usize {
        for j in 0..=n as usize {
            b.push(a[i] + a[j])
        }
    }

    b.sort_by(|a, b| b.cmp(a));

    for bb in &b {
        let bs = BinarySearch {
            p: |x| {
                b[x as usize] <= m - *bb
            },
            left: 0,
            right: (b.len()-1) as isize
        };
        let lb = bs.lower_bound();
        if lb != -1 {
            chmax!(ans, b[bs.lower_bound() as usize] + bb);
        }
    }
    println!("{}", ans)
}
