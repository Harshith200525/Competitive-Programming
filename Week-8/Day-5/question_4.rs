// Question 4: Palindrome Partitioning III
// How to partition a string into exactly k palindromic subsequences with the minimum number of changes required?

fn question_4(s: String, k: i32) -> i32 {
    let n = s.len();
    let s = s.as_bytes();
    let mut dp = vec![vec![0; n]; n];
    
    // Function to calculate minimum changes to make a substring a palindrome
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            dp[i][j] = if s[i] == s[j] {
                dp[i + 1][j - 1]
            } else {
                dp[i + 1][j - 1] + 1
            };
        }
    }

    // DP for partitioning the string into k palindromes
    let mut partition_dp = vec![vec![i32::MAX; k as usize + 1]; n];
    for i in 0..n {
        partition_dp[i][1] = dp[0][i];
    }

    for m in 2..=k as usize {
        for i in 1..n {
            for j in 0..i {
                partition_dp[i][m] = partition_dp[i][m].min(partition_dp[j][m - 1] + dp[j + 1][i]);
            }
        }
    }

    partition_dp[n - 1][k as usize]
}
