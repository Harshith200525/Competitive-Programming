// Question 1: Number of Distinct Islands
// How to count the number of distinct islands in a 2D grid where islands are defined by their shape, regardless of location?

use std::collections::HashSet;

fn question_1(grid: Vec<Vec<i32>>) -> i32 {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut islands = HashSet::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 && !visited[i][j] {
                let mut shape = Vec::new();
                dfs(i, j, &grid, &mut visited, &mut shape);
                islands.insert(shape);
            }
        }
    }

    islands.len() as i32
}

fn dfs(i: usize, j: usize, grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, shape: &mut Vec<(i32, i32)>) {
    if i < 0 || i >= grid.len() || j < 0 || j >= grid[0].len() || visited[i][j] || grid[i][j] == 0 {
        return;
    }

    visited[i][j] = true;
    shape.push((i as i32, j as i32));

    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    for (dx, dy) in directions {
        dfs((i as i32 + dx) as usize, (j as i32 + dy) as usize, grid, visited, shape);
    }
}
