// Question 1: Number of Good Paths
// How to find the number of valid paths in a graph based on specific constraints?

use std::collections::HashMap;

fn question_1(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    let n = vals.len();
    let mut graph = vec![vec![]; n];
    let mut union_find = (0..n).collect::<Vec<_>>();
    let mut size = vec![1; n];

    for edge in edges {
        graph[edge[0] as usize].push(edge[1] as usize);
        graph[edge[1] as usize].push(edge[0] as usize);
    }

    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }

    let mut nodes = (0..n).collect::<Vec<_>>();
    nodes.sort_by_key(|&x| vals[x]);

    let mut result = 0;

    for &node in &nodes {
        let value = vals[node];
        for &neighbor in &graph[node] {
            if vals[neighbor] <= value {
                let root1 = find(&mut union_find, node);
                let root2 = find(&mut union_find, neighbor);

                if root1 != root2 {
                    union_find[root1] = root2;
                    size[root2] += size[root1];
                }
            }
        }

        let root = find(&mut union_find, node);
        result += size[root];
    }

    result
}
