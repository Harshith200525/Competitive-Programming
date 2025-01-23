// Question 3: Palindrome Partitioning II
// How to minimize cuts required to partition a string into palindromes using dynamic programming?

fn question_3(s: String) -> i32 {
    let n = s.len();
    let s = s.chars().collect::<Vec<_>>();
    let mut dp = vec![vec![false; n]; n];

    for len in 1..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            dp[i][j] = s[i] == s[j] && (len <= 2 || dp[i + 1][j - 1]);
        }
    }

    let mut cuts = vec![i32::MAX; n];
    for i in 0..n {
        if dp[0][i] {
            cuts[i] = 0;
        } else {
            for j in 0..i {
                if dp[j + 1][i] {
                    cuts[i] = cuts[i].min(cuts[j] + 1);
                }
            }
        }
    }

    cuts[n - 1]
}
