// Question 4: Tallest Billboard
// How to maintain two billboards of equal height using a set of rods, ensuring the heights stay balanced?

fn question_4(rods: Vec<i32>) -> i32 {
    let mut dp = vec![0; 5001]; // Maximum height offset
    let offset = 2500; // Offset to handle negative values

    for &rod in &rods {
        let mut temp_dp = dp.clone();
        for i in 0..dp.len() {
            if dp[i] > 0 || i == 0 {
                if i + rod as usize < dp.len() {
                    temp_dp[i + rod as usize] = temp_dp[i + rod as usize].max(dp[i] + rod);
                }
                if i >= rod as usize {
                    temp_dp[i - rod as usize] = temp_dp[i - rod as usize].max(dp[i] + rod);
                }
            }
        }
        dp = temp_dp;
    }

    dp[offset]
}
