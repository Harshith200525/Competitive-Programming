// Question 1: Decode Ways II
// How to decode a string with additional constraints on valid codes using dynamic programming?

fn question_1(s: String) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let s = s.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 1..=n {
        if s[i - 1] != '0' {
            dp[i] = (dp[i] + dp[i - 1]) % MOD;
        }

        if i > 1 {
            let two_digit = (s[i - 2] as i32 - '0' as i32) * 10 + (s[i - 1] as i32 - '0' as i32);
            if two_digit >= 10 && two_digit <= 26 {
                dp[i] = (dp[i] + dp[i - 2]) % MOD;
            }
        }
    }

    dp[n]
}
