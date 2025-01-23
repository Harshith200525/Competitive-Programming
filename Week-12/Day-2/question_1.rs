// Question 1: Stone Game II
// How to find the maximum score achievable in a game involving piles of stones using dynamic programming?

fn question_1(piles: Vec<i32>) -> i32 {
    let n = piles.len();
    let mut suffix_sum = vec![0; n + 1];
    let mut dp = vec![vec![0; n + 1]; n];

    for i in (0..n).rev() {
        suffix_sum[i] = suffix_sum[i + 1] + piles[i];
    }

    for m in (1..=n).rev() {
        for i in (0..n).rev() {
            if i + 2 * m >= n {
                dp[i][m] = suffix_sum[i];
            } else {
                for x in 1..=2 * m {
                    dp[i][m] = dp[i][m].max(suffix_sum[i] - dp[i + x][x.max(m)]);
                }
            }
        }
    }

    dp[0][1]
}
