use proconio::input;

fn main() {
    input! {
        n: usize,
        mut red: [(usize,usize); n],
        mut blue: [(usize, usize); n]
    }
    blue.sort_by_key(|k| k.0);

    let mut cnt = 0;
    for j in 0..n {
        // retain red only less than blue.X
        let mut r = red.clone();
        r.retain(|(x, _)| *x < blue[j].0);
        r.sort_by_key(|k| k.1);

        // if no candidates, skip
        if r.is_empty() {
            continue;
        }

        // remove red that has the biggest y
        let (_, y) = r.pop().unwrap();
        red.remove(red.iter().position(|&xy| xy.1 == y).unwrap());
        cnt += 1
    }
    println!("{}", cnt);
}
