// Question 2: Count Different Palindromic Subsequences
// How to count all distinct palindromic subsequences in a string using dynamic programming and modular arithmetic?

fn question_2(s: String) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let s = s.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut dp = vec![vec![0; n]; n];

    for len in 1..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            if i == j {
                dp[i][j] = 1;
            } else {
                dp[i][j] = dp[i + 1][j] + dp[i][j - 1] - dp[i + 1][j - 1];
                if s[i] == s[j] {
                    dp[i][j] = (dp[i][j] + dp[i + 1][j - 1] + 1) % MOD;
                }
            }
        }
    }

    dp[0][n - 1]
}
