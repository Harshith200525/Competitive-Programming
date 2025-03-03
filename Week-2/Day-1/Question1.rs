// 1140. Stone Game II

// Alice and Bob continue their games with piles of stones. There are a number of piles arranged in a row, and each pile has a positive integer number of stones piles[i]. The objective of the game is to end with the most stones.

// Alice and Bob take turns, with Alice starting first.

// On each player's turn, that player can take all the stones in the first X remaining piles, where 1 <= X <= 2M. Then, we set M = max(M, X). Initially, M = 1.

// The game continues until all the stones have been taken.

// Assuming Alice and Bob play optimally, return the maximum number of stones Alice can get.

 

// Example 1:

// Input: piles = [2,7,9,4,4]

// Output: 10

// Explanation:

// If Alice takes one pile at the beginning, Bob takes two piles, then Alice takes 2 piles again. Alice can get 2 + 4 + 4 = 10 stones in total.
// If Alice takes two piles at the beginning, then Bob can take all three piles left. In this case, Alice get 2 + 7 = 9 stones in total.
// So we return 10 since it's larger.

// Example 2:

// Input: piles = [1,2,3,4,5,100]

// Output: 104

 

// Constraints:

// 1 <= piles.length <= 100
// 1 <= piles[i] <= 104

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();

        let mut dp = vec![vec![0; n+1]; n];

        let mut suffix_sum = vec![0; n];
        suffix_sum[n - 1] = piles[n - 1];

        for i in (0..n-1).rev() {
            suffix_sum[i] = suffix_sum[i + 1] + piles[i];
        }

        for i in (0..n).rev() {
            for m in 1..=n {
                if i + 2 * m >= n {
                    dp[i][m] = suffix_sum[i];
                } else {
                    for x in 1..=2 * m {
                        dp[i][m] = dp[i][m].max(suffix_sum[i] - dp[i + x][m.max(x)]);
                    }
                }
            }
        }

        return dp[0][1];
    }
}