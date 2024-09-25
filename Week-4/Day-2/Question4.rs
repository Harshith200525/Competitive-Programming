// 115. Distinct Subsequences

// Given two strings s and t, return the number of distinct subsequences of s which equals t.

// The test cases are generated so that the answer fits on a 32-bit signed integer.

 

// Example 1:

// Input: s = "rabbbit", t = "rabbit"
// Output: 3
// Explanation:
// As shown below, there are 3 ways you can generate "rabbit" from s.
// rabbbit
// rabbbit
// rabbbit
// Example 2:

// Input: s = "babgbag", t = "bag"
// Output: 5
// Explanation:
// As shown below, there are 5 ways you can generate "bag" from s.
// babgbag
// babgbag
// babgbag
// babgbag
// babgbag
 

// Constraints:

// 1 <= s.length, t.length <= 1000
// s and t consist of English letters.

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let mut dp = vec![vec![-1; t.len() + 1]; s.len() + 1];

        fn solve(
            i: usize,
            j: usize,
            s: &[u8],
            t: &[u8],
            dp: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if j == t.len() {
                return 1;
            } else if i == s.len() {
                return 0;
            }

            if dp[i][j] != -1 {
                return dp[i][j];
            }

            if s[i] == t[j] {
                dp[i][j] = solve(i + 1, j + 1, s, t, dp) + solve(i + 1, j, s, t, dp);
            } else {
                dp[i][j] = solve(i + 1, j, s, t, dp);
            }

            dp[i][j]
        }

        solve(0, 0, &s_bytes, &t_bytes, &mut dp)
    }
}
