// 62. Unique Paths

// There is a robot on an m x n grid. The robot is initially located at the top-left corner (i.e., grid[0][0]). The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only move either down or right at any point in time.

// Given the two integers m and n, return the number of possible unique paths that the robot can take to reach the bottom-right corner.

// The test cases are generated so that the answer will be less than or equal to 2 * 109.

// Input: m = 3, n = 2
// Output: 3
// Explanation: From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
// 1. Right -> Down -> Down
// 2. Down -> Down -> Right
// 3. Down -> Right -> Down

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut above_row = vec![1; n as usize];

        for _ in 1..m {
            let mut current_row = vec![1; n as usize];
            for i in 1..n as usize {
                current_row[i] = current_row[i - 1] + above_row[i];
            }
            above_row = current_row;
        }

        return *above_row.last().unwrap();
    }
}