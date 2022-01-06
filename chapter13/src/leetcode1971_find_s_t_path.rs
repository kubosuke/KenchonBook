// https://leetcode.com/problems/find-if-path-exists-in-graph/

use std::collections::HashSet;

fn main() {
    let n= 10;
    let edges: Vec<Vec<i32>> = vec![vec![0,7],vec![0,8],vec![6,1],vec![2,0],vec![0,4],vec![5,8],vec![4,7],vec![1,3],vec![3,5],vec![6,5]];
    let start = 7;
    let end= 5;

    println!("{}", valid_path(n, edges, start, end));
}

pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, start: i32, end: i32) -> bool {
    let graph: Vec<HashSet<usize>> = {
        let mut tmp: Vec<HashSet<usize>> = vec![HashSet::new(); n as usize];
        // O(|E|)
        for edge in edges.iter(){
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            tmp[u].insert(v);
            tmp[v].insert(u);
        }
        tmp
    };
    let mut seen: HashSet<usize> = HashSet::new();
    seen.insert(start as usize);
    dfs(&mut seen,  start as usize, end as usize, &graph)
}

fn dfs(seen: &mut HashSet<usize>, cur: usize, end: usize, graph: &Vec<HashSet<usize>>) -> bool {
    if cur == end {
        return true
    }
    for &next in &graph[cur] {
        if seen.insert(next) {
            if dfs(seen, next, end, graph) {
                return true
            }
        }
    };
    false
}
