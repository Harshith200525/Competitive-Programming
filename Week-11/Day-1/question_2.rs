// Question 2: All Paths From Source to Target
// How to find all possible paths from the source to the target in a directed acyclic graph?

fn question_2(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut path = Vec::new();

    fn dfs(node: i32, graph: &Vec<Vec<i32>>, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        path.push(node);

        if node as usize == graph.len() - 1 {
            result.push(path.clone());
        } else {
            for &next in &graph[node as usize] {
                dfs(next, graph, path, result);
            }
        }

        path.pop();
    }

    dfs(0, &graph, &mut path, &mut result);
    result
}
