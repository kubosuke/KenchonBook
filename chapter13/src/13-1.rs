// https://www.hackerearth.com/ja/problem/algorithm/connected-components-in-a-graph/
// input:
// 8 5
// 1 2
// 2 3
// 2 4
// 3 5
// 6 7
//
// output:
// 3

use std::collections::HashSet;

// proconio cannot use in hackerearth hence using stdin macro
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    input! {
        n: usize,
        m: usize,
        graph: [(usize, usize); m],
    }

    let hash_set: Vec<HashSet<usize>> = {
        let mut tmp: Vec<HashSet<usize>> = vec![HashSet::new(); n as usize];
        for edge in graph {
            tmp[edge.0 - 1].insert(edge.1 - 1);
        }
        tmp
    };

    let mut seen: HashSet<usize> = HashSet::new();

    let mut cnt = 0;
    for v in 0..n {
        if seen.get(&v) != None {
            continue
        }
        dfs(&hash_set, v, &mut seen);

        cnt += 1;
    }

    println!("{}", cnt);
}

fn dfs(graph: &Vec<HashSet<usize>>, v: usize,  seen: &mut HashSet<usize>) {
    for to in &graph[v] {
        if seen.insert(*to) {
            dfs(graph, *to, seen)
        }
    }
}
