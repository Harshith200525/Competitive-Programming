// Question 1: Unique Paths III
// How to find all possible paths to traverse a grid such that all blank cells are visited exactly once before reaching the destination?

fn question_1(grid: Vec<Vec<i32>>) -> i32 {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut empty_cells = 0;

    // Find start, end, and count empty cells
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                start = (i, j);
            } else if grid[i][j] == 2 {
                end = (i, j);
            } else if grid[i][j] == 0 {
                empty_cells += 1;
            }
        }
    }

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    visited[start.0][start.1] = true;
    let mut paths = 0;
    
    fn backtrack(i: usize, j: usize, visited: &mut Vec<Vec<bool>>, empty_cells: i32, grid: &Vec<Vec<i32>>, paths: &mut i32) {
        if (i, j) == end {
            if visited.iter().flatten().filter(|&&cell| !cell).count() as i32 == empty_cells {
                *paths += 1;
            }
            return;
        }
        
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions {
            let new_i = (i as i32 + dx) as usize;
            let new_j = (j as i32 + dy) as usize;
            
            if new_i < grid.len() && new_j < grid[0].len() && !visited[new_i][new_j] && grid[new_i][new_j] != -1 {
                visited[new_i][new_j] = true;
                backtrack(new_i, new_j, visited, empty_cells, grid, paths);
                visited[new_i][new_j] = false;
            }
        }
    }

    backtrack(start.0, start.1, &mut visited, empty_cells, &grid, &mut paths);
    paths
}
