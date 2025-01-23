// Question 4: Unique Paths II
// How to find the number of unique paths in a grid with obstacles using Rust?

fn question_4(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut dp = vec![vec![0; n]; m];

    if grid[0][0] == 1 {
        return 0;
    }

    dp[0][0] = 1;

    for i in 1..m {
        dp[i][0] = if grid[i][0] == 1 { 0 } else { dp[i - 1][0] };
    }

    for j in 1..n {
        dp[0][j] = if grid[0][j] == 1 { 0 } else { dp[0][j - 1] };
    }

    for i in 1..m {
        for j in 1..n {
            if grid[i][j] == 1 {
                dp[i][j] = 0;
            } else {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
    }

    dp[m - 1][n - 1]
}
