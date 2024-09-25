// 329. Longest Increasing Path in a Matrix

// Given an m x n integers matrix, return the length of the longest increasing path in matrix.

// From each cell, you can either move in four directions: left, right, up, or down. You may not move diagonally or move outside the boundary (i.e., wrap-around is not allowed).

 

// Example 1:


// Input: matrix = [[9,9,4],[6,6,8],[2,1,1]]
// Output: 4
// Explanation: The longest increasing path is [1, 2, 6, 9].
// Example 2:


// Input: matrix = [[3,4,5],[3,2,6],[2,2,1]]
// Output: 4
// Explanation: The longest increasing path is [3, 4, 5, 6]. Moving diagonally is not allowed.
// Example 3:

// Input: matrix = [[1]]
// Output: 1
 

// Constraints:

// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 200
// 0 <= matrix[i][j] <= 231 - 1

use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        if n == 0 || m == 0 {
            return 0;
        }
        
        let mut dp = vec![vec![-1; m]; n];
        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        fn helper(r: usize, c: usize, matrix: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, directions: &Vec<(isize, isize)>) -> i32 {
            if dp[r][c] != -1 {
                return dp[r][c];
            }
            
            let mut ans = 1;
            for &(dr, dc) in directions.iter() {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                
                if nr >= 0 && nr < matrix.len() as isize && nc >= 0 && nc < matrix[0].len() as isize {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    
                    if matrix[nr][nc] > matrix[r][c] {
                        ans = max(ans, 1 + helper(nr, nc, matrix, dp, directions));
                    }
                }
            }
            dp[r][c] = ans;
            ans
        }

        let mut max_length = 0;
        for i in 0..n {
            for j in 0..m {
                max_length = max(max_length, helper(i, j, &matrix, &mut dp, &directions));
            }
        }
        max_length
    }
}
