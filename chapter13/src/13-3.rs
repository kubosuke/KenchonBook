// https://leetcode.com/problems/is-graph-bipartite/
use std::collections::VecDeque;

fn main() {
    let edges: Vec<Vec<i32>> = vec![vec![1,2,3],vec![0,2],vec![0,1,3],vec![0,2]];
    assert_eq!(false, is_bipartite(edges));

    let edges: Vec<Vec<i32>> = vec![vec![1,3],vec![0,2],vec![1,3],vec![0,2]];
    assert_eq!(true, is_bipartite(edges));

    let edges: Vec<Vec<i32>> = vec![vec![],vec![2,4,6],vec![1,4,8,9],vec![7,8],vec![1,2,8,9],vec![6,9],vec![1,5,7,8,9],vec![3,6,9],vec![2,3,4,6,9],vec![2,4,5,6,7,8]];
    assert_eq!(false, is_bipartite(edges))
}

pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
    let mut color = vec![None; graph.len()];

    for f in 0..color.len() {
        if color[f].is_some() {
            continue
        }
        color[f] = Some(true);

        let mut vd = VecDeque::new();
        vd.push_front(f);

        while let Some(v) = vd.pop_back() {
            for t in graph[v].iter().map(|&x| x as usize) {
                if color[v] == color[t] {
                    return false
                }
                if color[t].is_none() {
                    color[t] = color[v].map(|x|!x);
                    vd.push_front(t);
                }
            }
        }
    }
    true
}
