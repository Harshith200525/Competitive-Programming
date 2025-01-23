// Question 1: Super Egg Drop
// How to determine the minimum number of moves required to find the critical floor in a building using k eggs?

fn question_1(k: i32, n: i32) -> i32 {
    let mut dp = vec![vec![0; (n + 1) as usize]; (k + 1) as usize];

    for m in 1..=n {
        for e in 1..=k {
            dp[e as usize][m as usize] = 1 + dp[(e - 1) as usize][(m - 1) as usize] + dp[e as usize][(m - 1) as usize];
            if dp[e as usize][m as usize] >= n {
                return m;
            }
        }
    }

    n
}
