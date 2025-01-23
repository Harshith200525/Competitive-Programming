// Question 4: Number of Unique Paths
// How to find the number of unique paths from the top-left corner to the bottom-right corner of a grid using Rust?

fn question_4(m: i32, n: i32) -> i32 {
    let mut dp = vec![vec![1; n as usize]; m as usize];

    for i in 1..m as usize {
        for j in 1..n as usize {
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }

    dp[m as usize - 1][n as usize - 1]
}
