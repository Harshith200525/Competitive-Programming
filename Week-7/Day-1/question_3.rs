// Question 3: Decode Ways
// How to find the number of ways a numeric string can be decoded into letters using Rust?

fn question_3(s: String) -> i32 {
    if s.is_empty() || s.starts_with('0') {
        return 0;
    }

    let mut dp = vec![0; s.len() + 1];
    dp[0] = 1;
    dp[1] = 1;

    for i in 2..=s.len() {
        let one_digit: i32 = s[i - 1..i].parse().unwrap();
        let two_digits: i32 = s[i - 2..i].parse().unwrap();

        if one_digit >= 1 {
            dp[i] += dp[i - 1];
        }
        if two_digits >= 10 && two_digits <= 26 {
            dp[i] += dp[i - 2];
        }
    }

    dp[s.len()]
}
