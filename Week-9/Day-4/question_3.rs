// Question 3: Stone Game VII
// How to maximize scores while removing stones in a game using dynamic programming?

fn question_3(stones: Vec<i32>) -> i32 {
    let n = stones.len();
    let mut prefix_sum = vec![0; n + 1];
    let mut dp = vec![vec![0; n]; n];

    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + stones[i];
    }

    for length in 2..=n {
        for i in 0..=n - length {
            let j = i + length - 1;
            dp[i][j] = std::cmp::max(prefix_sum[j + 1] - prefix_sum[i + 1] - dp[i + 1][j], prefix_sum[j] - prefix_sum[i] - dp[i][j - 1]);
        }
    }

    dp[0][n - 1]
}
