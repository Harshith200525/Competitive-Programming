// 200. Number of Islands

// Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water), return the number of islands.

// An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.

 

// Example 1:

// Input: grid = [
//   ["1","1","1","1","0"],
//   ["1","1","0","1","0"],
//   ["1","1","0","0","0"],
//   ["0","0","0","0","0"]
// ]
// Output: 1
// Example 2:

// Input: grid = [
//   ["1","1","0","0","0"],
//   ["1","1","0","0","0"],
//   ["0","0","1","0","0"],
//   ["0","0","0","1","1"]
// ]
// Output: 3
 

// Constraints:

// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 300
// grid[i][j] is '0' or '1'.

use std::collections::VecDeque;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }

        let rows = grid.len();
        let cols = grid[0].len();
        let mut visited = vec![vec![false; cols]; rows];
        let mut islands = 0;

        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        fn bfs(r: usize, c: usize, grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) {
            let mut queue = VecDeque::new();
            queue.push_back((r, c));
            visited[r][c] = true;

            while let Some((row, col)) = queue.pop_front() {
                for (dr, dc) in &directions {
                    let new_row = (row as isize + dr) as usize;
                    let new_col = (col as isize + dc) as usize;

                    if new_row < grid.len() && new_col < grid[0].len() && grid[new_row][new_col] == '1' && !visited[new_row][new_col] {
                        queue.push_back((new_row, new_col));
                        visited[new_row][new_col] = true;
                    }
                }
            }
        }

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '1' && !visited[r][c] {
                    islands += 1;
                    bfs(r, c, &grid, &mut visited);
                }
            }
        }

        islands
    }
}
