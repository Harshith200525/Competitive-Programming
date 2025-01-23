// Question 3: Evaluate Division
// How to evaluate division equations using graph traversal in Rust?

use std::collections::HashMap;

fn question_3(
    equations: Vec<(String, String)>,
    values: Vec<f64>,
    queries: Vec<(String, String)>,
) -> Vec<f64> {
    let mut graph = HashMap::new();

    for ((a, b), &val) in equations.iter().zip(&values) {
        graph.entry(a.clone()).or_insert_with(Vec::new).push((b.clone(), val));
        graph.entry(b.clone()).or_insert_with(Vec::new).push((a.clone(), 1.0 / val));
    }

    fn dfs(
        graph: &HashMap<String, Vec<(String, f64)>>,
        current: &String,
        target: &String,
        visited: &mut HashMap<String, bool>,
    ) -> Option<f64> {
        if let Some(edges) = graph.get(current) {
            if current == target {
                return Some(1.0);
            }
            visited.insert(current.clone(), true);
            for (neighbor, &weight) in edges {
                if !visited.get(neighbor).unwrap_or(&false) {
                    if let Some(product) = dfs(graph, neighbor, target, visited) {
                        return Some(product * weight);
                    }
                }
            }
        }
        None
    }

    queries
        .iter()
        .map(|(a, b)| {
            if !graph.contains_key(a) || !graph.contains_key(b) {
                -1.0
            } else {
                dfs(&graph, a, b, &mut HashMap::new()).unwrap_or(-1.0)
            }
        })
        .collect()
}
