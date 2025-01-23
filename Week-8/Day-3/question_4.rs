// Question 4: Shortest Path in a Grid with Obstacles Elimination
// How to find the shortest path in a grid while being allowed to eliminate a certain number of obstacles?

use std::collections::VecDeque;

fn question_4(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    
    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, k, 0)); // (x, y, remaining_k, steps)
    
    let mut visited = vec![vec![vec![-1; n]]; m];
    visited[0][0][k as usize] = 0;
    
    while let Some((x, y, remaining_k, steps)) = queue.pop_front() {
        if x == m - 1 && y == n - 1 {
            return steps;
        }
        
        for (dx, dy) in directions {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            
            if nx < m && ny < n {
                if grid[nx][ny] == 1 && remaining_k > 0 && visited[nx][ny][(remaining_k - 1) as usize] == -1 {
                    visited[nx][ny][(remaining_k - 1) as usize] = steps + 1;
                    queue.push_back((nx, ny, remaining_k - 1, steps + 1));
                } else if grid[nx][ny] == 0 && visited[nx][ny][remaining_k as usize] == -1 {
                    visited[nx][ny][remaining_k as usize] = steps + 1;
                    queue.push_back((nx, ny, remaining_k, steps + 1));
                }
            }
        }
    }
    
    -1
}
