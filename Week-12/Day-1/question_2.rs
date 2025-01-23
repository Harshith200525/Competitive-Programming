// Question 2: Dungeon Game
// How to calculate the minimum health needed to traverse a dungeon using dynamic programming?

fn question_2(dungeon: Vec<Vec<i32>>) -> i32 {
    let rows = dungeon.len();
    let cols = dungeon[0].len();
    let mut dp = vec![vec![i32::MAX; cols + 1]; rows + 1];
    dp[rows][cols - 1] = 1;
    dp[rows - 1][cols] = 1;

    for i in (0..rows).rev() {
        for j in (0..cols).rev() {
            let min_health = dp[i + 1][j].min(dp[i][j + 1]) - dungeon[i][j];
            dp[i][j] = min_health.max(1);
        }
    }

    dp[0][0]
}
