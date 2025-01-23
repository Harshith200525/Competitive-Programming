// Question 4: Shortest Path in a Grid with Obstacles Elimination
// How to navigate a grid while removing a limited number of obstacles using BFS with state tracking?

use std::collections::{VecDeque, HashSet};

fn question_4(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut queue = VecDeque::new();
    let mut visited = vec![vec![vec![false; k as usize + 1]; m]; n];

    queue.push_back((0, 0, 0)); // (x, y, obstacles_removed)
    visited[0][0][0] = true;

    let mut steps = 0;

    while !queue.is_empty() {
        let size = queue.len();
        for _ in 0..size {
            let (x, y, obs) = queue.pop_front().unwrap();
            if x == n - 1 && y == m - 1 {
                return steps;
            }

            for (dx, dy) in &directions {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                let next_obs = if grid[nx][ny] == 1 { obs + 1 } else { obs };

                if nx < n && ny < m && next_obs <= k as usize && !visited[nx][ny][next_obs] {
                    visited[nx][ny][next_obs] = true;
                    queue.push_back((nx, ny, next_obs));
                }
            }
        }
        steps += 1;
    }

    -1
}
