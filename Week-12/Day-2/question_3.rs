// Question 3: Minimum Cost to Merge Stones
// How to find the minimum cost to merge stones into a single pile using memoized recursion?

fn question_3(stones: Vec<i32>, k: i32) -> i32 {
    let n = stones.len();
    if (n - 1) % (k as usize - 1) != 0 {
        return -1;
    }

    let mut prefix_sum = vec![0; n + 1];
    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + stones[i];
    }

    let mut dp = vec![vec![0; n]; n];
    for length in k as usize..=n {
        for i in 0..=n - length {
            let j = i + length - 1;
            dp[i][j] = i32::MAX;
            for mid in (i..j).step_by(k as usize - 1) {
                dp[i][j] = dp[i][j].min(dp[i][mid] + dp[mid + 1][j]);
            }
            if (length - 1) % (k as usize - 1) == 0 {
                dp[i][j] += prefix_sum[j + 1] - prefix_sum[i];
            }
        }
    }

    dp[0][n - 1]
}
