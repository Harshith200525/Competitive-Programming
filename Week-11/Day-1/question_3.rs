// Question 3: Best Time to Buy and Sell Stock IV
// How to maximize profit with a limited number of stock transactions using dynamic programming?

fn question_3(k: i32, prices: Vec<i32>) -> i32 {
    let n = prices.len();
    if n == 0 || k == 0 {
        return 0;
    }

    if k as usize >= n / 2 {
        return prices.windows(2).map(|w| (w[1] - w[0]).max(0)).sum();
    }

    let mut dp = vec![vec![0; n]; (k + 1) as usize];
    for t in 1..=k as usize {
        let mut max_diff = -prices[0];
        for d in 1..n {
            dp[t][d] = dp[t][d - 1].max(prices[d] + max_diff);
            max_diff = max_diff.max(dp[t - 1][d] - prices[d]);
        }
    }

    dp[k as usize][n - 1]
}
