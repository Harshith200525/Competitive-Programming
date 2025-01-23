// Question 3: Sum of Distances in Tree
// How to compute the sum of distances between all pairs of nodes in a tree efficiently?

fn question_3(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut tree = vec![vec![]; n];
    for edge in &edges {
        tree[edge[0] as usize].push(edge[1] as usize);
        tree[edge[1] as usize].push(edge[0] as usize);
    }

    let mut count = vec![1; n];
    let mut result = vec![0; n];

    fn dfs(node: usize, parent: usize, tree: &Vec<Vec<usize>>, count: &mut Vec<i32>, result: &mut Vec<i32>) {
        for &neighbor in &tree[node] {
            if neighbor == parent {
                continue;
            }
            dfs(neighbor, node, tree, count, result);
            count[node] += count[neighbor];
            result[node] += result[neighbor] + count[neighbor];
        }
    }

    fn dfs2(node: usize, parent: usize, tree: &Vec<Vec<usize>>, count: &Vec<i32>, result: &mut Vec<i32>) {
        for &neighbor in &tree[node] {
            if neighbor == parent {
                continue;
            }
            result[neighbor] = result[node] - count[neighbor] + (count.len() as i32 - count[neighbor]);
            dfs2(neighbor, node, tree, count, result);
        }
    }

    dfs(0, usize::MAX, &tree, &mut count, &mut result);
    dfs2(0, usize::MAX, &tree, &count, &mut result);

    result
}
