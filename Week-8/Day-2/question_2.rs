// Question 2: Palindrome Partitioning II
// How to find the minimum number of cuts needed to split a string into palindromes using Rust?

fn question_2(s: String) -> i32 {
    let n = s.len();
    let s = s.as_bytes();
    let mut dp = vec![i32::MAX; n];
    let mut is_palindrome = vec![vec![false; n]; n];

    // Mark all substrings that are palindromes
    for i in (0..n).rev() {
        for j in i..n {
            if s[i] == s[j] && (j - i <= 2 || is_palindrome[i + 1][j - 1]) {
                is_palindrome[i][j] = true;
            }
        }
    }

    // DP to calculate the minimum cuts
    for i in 0..n {
        if is_palindrome[0][i] {
            dp[i] = 0;
        } else {
            for j in 0..i {
                if is_palindrome[j + 1][i] {
                    dp[i] = dp[i].min(dp[j] + 1);
                }
            }
        }
    }

    dp[n - 1]
}
