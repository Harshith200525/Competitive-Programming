// Question 2: Super Egg Drop
// How to determine the minimum number of trials required to find the highest floor an egg can be dropped from without breaking, given k eggs and n floors?

fn question_2(k: i32, n: i32) -> i32 {
    let mut dp = vec![vec![0; (k + 1) as usize]; (n + 1) as usize];
    
    for i in 0..=n as usize {
        dp[i][0] = 0;
        dp[i][1] = i as i32;
    }
    
    for i in 0..=k as usize {
        dp[0][i] = 0;
        dp[1][i] = 1;
    }
    
    for i in 2..=n as usize {
        for j in 2..=k as usize {
            dp[i][j] = i32::MAX;
            for x in 1..=i {
                dp[i][j] = dp[i][j].min(1 + std::cmp::max(dp[x - 1][j - 1], dp[i - x][j]));
            }
        }
    }
    
    dp[n as usize][k as usize]
}
