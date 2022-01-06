// https://leetcode.com/problems/find-if-path-exists-in-graph/

use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let n= 10;
    let edges: Vec<Vec<i32>> = vec![vec![0,7],vec![0,8],vec![6,1],vec![2,0],vec![0,4],vec![5,8],vec![4,7],vec![1,3],vec![3,5],vec![6,5]];
    let start = 7;
    let end= 5;

    assert_eq!(true, valid_path(n, edges, start, end));
}

pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, start: i32, end: i32) -> bool {
    let graph: Vec<HashSet<i32>> = {
        let mut tmp: Vec<HashSet<i32>> = vec![HashSet::new(); n as usize];
        // O(|E|)
        for edge in edges.iter(){
            let u = edge[0];
            let v = edge[1];
            tmp[u as usize].insert(v);
            tmp[v as usize].insert(u);
        }
        tmp
    };

    let mut seen = HashSet::new();
    seen.insert(start);

    let mut queue = VecDeque::new();
    queue.push_front(start);

    while !queue.is_empty() {
        match queue.pop_back() {
            Some(v) => {
                if v == end {
                    return true
                }
                for to in &graph[v as usize] {
                    if seen.insert(*to) {
                        queue.push_front(*to);
                    }
                }
            },
            None => continue
        }
    };
    false
}
