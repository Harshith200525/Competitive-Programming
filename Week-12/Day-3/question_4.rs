// Question 4: Wildcard Matching
// How to implement a pattern-matching algorithm with support for wildcards using dynamic programming?

fn question_4(s: String, p: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let p = p.chars().collect::<Vec<_>>();
    let m = s.len();
    let n = p.len();
    let mut dp = vec![vec![false; n + 1]; m + 1];

    dp[0][0] = true;
    for j in 1..=n {
        if p[j - 1] == '*' {
            dp[0][j] = dp[0][j - 1];
        }
    }

    for i in 1..=m {
        for j in 1..=n {
            if p[j - 1] == '*' {
                dp[i][j] = dp[i - 1][j] || dp[i][j - 1];
            } else if p[j - 1] == '?' || s[i - 1] == p[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }

    dp[m][n]
}
