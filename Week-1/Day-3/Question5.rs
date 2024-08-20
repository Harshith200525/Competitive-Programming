// 1568. Minimum Number of Days to Disconnect Island
// You are given an m x n binary grid grid where 1 represents land and 0 represents water. An island is a maximal 4-directionally (horizontal or vertical) connected group of 1's.

// The grid is said to be connected if we have exactly one island, otherwise is said disconnected.

// In one day, we are allowed to change any single land cell (1) into a water cell (0).

// Return the minimum number of days to disconnect the grid.

// Input: grid = [[0,1,1,0],[0,1,1,0],[0,0,0,0]]

// Output: 2
// Explanation: We need at least 2 days to get a disconnected grid.
// Change land grid[1][1] and grid[0][2] to water and get 2 disconnected island.

impl Solution {
    pub fn min_days(mut grid: Vec<Vec<i32>>) -> i32 {
        fn count_islands(grid: &Vec<Vec<i32>>) -> i32 {
            let mut seen = std::collections::HashSet::new();
            let mut islands = 0;

            fn dfs(r: usize, c: usize, grid: &Vec<Vec<i32>>, seen: &mut std::collections::HashSet<(usize, usize)>){
                let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
                let mut stack = vec![(r, c)];
                seen.insert((r,c));

                while let Some((x, y)) = stack.pop() {
                    for (dx, dy) in &directions {
                        let nx = (x as i32 + dx) as usize;
                        let ny = (y as i32 + dy) as usize;

                        if nx < grid.len() && ny < grid[0].len() && grid[nx][ny] == 1 && !seen.contains(&(nx, ny)){
                            seen.insert((nx, ny));
                            stack.push((nx, ny));
                        }
                    }
                }
            }

            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if grid[i][j] == 1 && !seen.contains(&(i, j)){
                        islands += 1;
                        dfs(i, j, grid, &mut seen);
                    }
                }
            }

            return islands;
        }

        if count_islands(&grid) != 1{
            return 0;
        }

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1{
                    grid[i][j] = 0;

                    if count_islands(&grid) != 1 {
                        return 1;
                    }

                    grid[i][j] = 1;
                }
            }
        }

        return 2;
    }
}