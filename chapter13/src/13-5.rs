// question: https://www.hackerearth.com/practice/algorithms/graphs/topological-sort/practice-problems/algorithm/lonelyisland-49054110/
// ref: https://leetcode.com/discuss/general-discussion/1078072/introduction-to-topological-sort

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
        r: usize, // 1-index
        paths: [(usize, usize); m] // 1-index
    }

    let r = r - 1; //0-index

    let graph: Vec<HashSet<usize>> = {
        let mut tmp: Vec<HashSet<usize>> = vec![HashSet::new(); n];
        for path in paths.iter() {
            tmp[path.0 - 1].insert(path.1 - 1); // 0-index
        }
        tmp
    };

    let mut q = vec![];
    q.push(r);

    let mut pq = vec![0.0; n];
    pq[r] = 1.0;

    while !q.is_empty() {
        let v = q.pop().unwrap();
        let l = graph[v].len();
        if graph[v].is_empty() {
            continue;
        }
        for next in &graph[v] {
            q.push(*next);
            pq[*next] += pq[v] / l as f64;
        }
        pq[v] = 0.0;
    }

    let mut m = 0.0;
    let mut a = vec![];
    for i in 0..n {
        if ((pq[i] - m) as f64).abs() <= 0.000000001 {
            a.push(i + 1);
            continue;
        }
        if pq[i] > m {
            a = vec![];
            a.push(i + 1);
            m = pq[i];
        }
    }
    a.sort();

    for aa in a {
        print!("{} ", aa);
    }
    println!();
}
