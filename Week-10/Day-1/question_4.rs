// Question 4: Evaluate Division
// How to compute results for equations based on division relationships?

use std::collections::HashMap;

fn question_4(
    equations: Vec<(String, String)>,
    values: Vec<f64>,
    queries: Vec<(String, String)>,
) -> Vec<f64> {
    let mut graph = HashMap::new();

    for ((a, b), &value) in equations.iter().zip(&values) {
        graph.entry(a.clone()).or_insert(Vec::new()).push((b.clone(), value));
        graph.entry(b.clone()).or_insert(Vec::new()).push((a.clone(), 1.0 / value));
    }

    fn dfs(
        graph: &HashMap<String, Vec<(String, f64)>>,
        current: &str,
        target: &str,
        visited: &mut HashSet<String>,
        acc: f64,
    ) -> Option<f64> {
        if current == target {
            return Some(acc);
        }

        visited.insert(current.to_string());
        for (neighbor, &value) in &graph[current] {
            if !visited.contains(neighbor) {
                if let Some(result) = dfs(graph, neighbor, target, visited, acc * value) {
                    return Some(result);
                }
            }
        }

        None
    }

    queries
        .into_iter()
        .map(|(a, b)| {
            if !graph.contains_key(&a) || !graph.contains_key(&b) {
                -1.0
            } else {
                dfs(&graph, &a, &b, &mut HashSet::new(), 1.0).unwrap_or(-1.0)
            }
        })
        .collect()
}
