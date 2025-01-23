// Question 1: Strange Printer
// How to determine the minimum number of turns to print a string using a peculiar printer?

fn question_1(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut dp = vec![vec![0; n]; n];

    for len in 1..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            dp[i][j] = len as i32; // Worst case: print each character individually
            for k in i..j {
                let cost = dp[i][k] + dp[k + 1][j];
                dp[i][j] = dp[i][j].min(if s[k] == s[j] { cost - 1 } else { cost });
            }
        }
    }

    dp[0][n - 1]
}
