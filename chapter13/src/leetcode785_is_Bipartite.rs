//

fn main() {
    let edges: Vec<Vec<i32>> = vec![vec![1,2,3],vec![0,2],vec![0,1,3],vec![0,2]];
    assert_eq!(false, is_bipartite(edges));

    let edges: Vec<Vec<i32>> = vec![vec![1,3],vec![0,2],vec![1,3],vec![0,2]];
    assert_eq!(true, is_bipartite(edges));
}

pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
    let n = graph.len();
    let mut color = vec![-1; n];
    for v in 0..n {
        if color[v] != -1 {
            continue
        }
        if !dfs(&graph, v, &mut color, 0) {
            return false
        }
    };
    true
}

fn dfs(graph: &Vec<Vec<i32>>, v: usize, color: &mut Vec<i32>, cur: i32) -> bool {
    color[v] = cur;

    for vv in &graph[v] {
        if color[*vv as usize] != -1 {
            if color[*vv as usize] == cur {
                return false
            }
            continue
        }
        if !dfs(graph, *vv as usize, color, (cur+1)%2) {
            return false
        }
    };
    true
}
