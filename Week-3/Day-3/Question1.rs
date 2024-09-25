// 2699. Modify Graph Edge Weights

// You are given an undirected weighted connected graph containing n nodes labeled from 0 to n - 1, and an integer array edges where edges[i] = [ai, bi, wi] indicates that there is an edge between nodes ai and bi with weight wi.

// Some edges have a weight of -1 (wi = -1), while others have a positive weight (wi > 0).

// Your task is to modify all edges with a weight of -1 by assigning them positive integer values in the range [1, 2 * 109] so that the shortest distance between the nodes source and destination becomes equal to an integer target. If there are multiple modifications that make the shortest distance between source and destination equal to target, any of them will be considered correct.

// Return an array containing all edges (even unmodified ones) in any order if it is possible to make the shortest distance from source to destination equal to target, or an empty array if it's impossible.

// Note: You are not allowed to modify the weights of edges with initial positive weights.

 

// Example 1:



// Input: n = 5, edges = [[4,1,-1],[2,0,-1],[0,3,-1],[4,3,-1]], source = 0, destination = 1, target = 5
// Output: [[4,1,1],[2,0,1],[0,3,3],[4,3,1]]
// Explanation: The graph above shows a possible modification to the edges, making the distance from 0 to 1 equal to 5.
// Example 2:



// Input: n = 3, edges = [[0,1,-1],[0,2,5]], source = 0, destination = 2, target = 6
// Output: []
// Explanation: The graph above contains the initial edges. It is not possible to make the distance from 0 to 2 equal to 6 by modifying the edge with weight -1. So, an empty array is returned.
// Example 3:



// Input: n = 4, edges = [[1,0,4],[1,2,3],[2,3,5],[0,3,-1]], source = 0, destination = 2, target = 6
// Output: [[1,0,4],[1,2,3],[2,3,5],[0,3,1]]
// Explanation: The graph above shows a modified graph having the shortest distance from 0 to 2 as 6.
 

// Constraints:

// 1 <= n <= 100
// 1 <= edges.length <= n * (n - 1) / 2
// edges[i].length == 3
// 0 <= ai, bi < n
// wi = -1 or 1 <= wi <= 107
// ai != bi
// 0 <= source, destination < n
// source != destination
// 1 <= target <= 109
// The graph is connected, and there are no self-loops or repeated edges

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn modified_graph_edges(
        n: i32, 
        mut edges: Vec<Vec<i32>>, 
        source: i32, 
        destination: i32, 
        target: i32
    ) -> Vec<Vec<i32>> {
        
        let n = n as usize;
        let source = source as usize;
        let destination = destination as usize;
        
        let mut adjacency_list: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
        for (i, edge) in edges.iter().enumerate() {
            let node_a = edge[0] as usize;
            let node_b = edge[1] as usize;
            adjacency_list[node_a].push((node_b, i));
            adjacency_list[node_b].push((node_a, i));
        }

        let mut distances = vec![vec![i32::MAX; 2]; n];
        distances[source][0] = 0;
        distances[source][1] = 0;

        Self::run_dijkstra(&adjacency_list, &mut edges, &mut distances, source, 0, 0);
        let difference = target - distances[destination][0];

        if difference < 0 {
            return vec![];
        }

        Self::run_dijkstra(&adjacency_list, &mut edges, &mut distances, source, difference, 1);

        if distances[destination][1] < target {
            return vec![];
        }

        for edge in edges.iter_mut() {
            if edge[2] == -1 {
                edge[2] = 1;
            }
        }

        edges
    }

    fn run_dijkstra(
        adjacency_list: &Vec<Vec<(usize, usize)>>, 
        edges: &mut Vec<Vec<i32>>, 
        distances: &mut Vec<Vec<i32>>, 
        source: usize, 
        difference: i32, 
        run: usize
    ) {
        let mut priority_queue: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
        priority_queue.push((Reverse(0), source));
        distances[source][run] = 0;

        while let Some((Reverse(current_distance), current_node)) = priority_queue.pop() {
            if current_distance > distances[current_node][run] {
                continue;
            }

            for &(next_node, edge_index) in &adjacency_list[current_node] {
                let mut weight = edges[edge_index][2];
                if weight == -1 {
                    weight = 1;
                }

                if run == 1 && edges[edge_index][2] == -1 {
                    let new_weight = difference + distances[next_node][0] - distances[current_node][1];
                    if new_weight > weight {
                        edges[edge_index][2] = weight = new_weight;
                    }
                }

                if distances[next_node][run] > distances[current_node][run] + weight {
                    distances[next_node][run] = distances[current_node][run] + weight;
                    priority_queue.push((Reverse(distances[next_node][run]), next_node));
                }
            }
        }
    }
}
