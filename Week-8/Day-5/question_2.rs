// Question 2: Find the Derangement of an Array
// How to find the number of ways to rearrange an array such that no element appears in its original position?

fn question_2(n: i32) -> i32 {
    let mut dp = vec![0; (n + 1) as usize];
    dp[0] = 1;
    dp[1] = 0;
    
    for i in 2..=n as usize {
        dp[i] = (i - 1) * (dp[i - 1] + dp[i - 2]);
    }

    dp[n as usize]
}
