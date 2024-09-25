// 64. Minimum Path Sum

// Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right, which minimizes the sum of all numbers along its path.

// Note: You can only move either down or right at any point in time.

 

// Example 1:


// Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
// Output: 7
// Explanation: Because the path 1 → 3 → 1 → 1 → 1 minimizes the sum.
// Example 2:

// Input: grid = [[1,2,3],[4,5,6]]
// Output: 12
 

// Constraints:

// m == grid.length
// n == grid[i].length
// 1 <= m, n <= 200
// 0 <= grid[i][j] <= 200

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    ans[i][j] = grid[i][j];
                } else if i == 0 {
                    ans[i][j] = grid[i][j] + ans[i][j - 1];
                } else if j == 0 {
                    ans[i][j] = grid[i][j] + ans[i- 1][j];
                } else {
                    ans[i][j] = grid[i][j] + std::cmp::min(ans[i - 1][j], ans[i][j - 1]);
                }
            }
        }

        return ans[m - 1][n - 1];
    }
}