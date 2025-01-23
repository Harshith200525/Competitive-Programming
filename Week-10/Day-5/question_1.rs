// Question 1: Edit Distance
// How to find the minimum number of operations to convert one string into another using dynamic programming?

fn question_1(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        for j in 0..=n {
            if i == 0 {
                dp[i][j] = j as i32;
            } else if j == 0 {
                dp[i][j] = i as i32;
            } else if word1.as_bytes()[i - 1] == word2.as_bytes()[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
            }
        }
    }

    dp[m][n]
}
