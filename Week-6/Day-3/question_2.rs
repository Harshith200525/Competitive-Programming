// Question 2: Unique Paths II
// How can we determine the number of unique paths in a grid with obstacles using Rust?

fn question_2(grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dp = vec![vec![0; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 1 {
                dp[i][j] = 0;
            } else if i == 0 && j == 0 {
                dp[i][j] = 1;
            } else {
                dp[i][j] = if i > 0 { dp[i - 1][j] } else { 0 } +
                           if j > 0 { dp[i][j - 1] } else { 0 };
            }
        }
    }
    dp[rows - 1][cols - 1]
}
