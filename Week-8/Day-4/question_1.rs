// Question 1: Minimum Cost to Merge Stones
// How to merge stones into one pile with the minimum cost, given the cost for each merge?

fn question_1(stones: Vec<i32>, k: i32) -> i32 {
    let n = stones.len();
    if (n - 1) % (k - 1) != 0 {
        return -1; // If it's not possible to merge into one pile
    }

    let mut dp = vec![vec![vec![-1; n]; n]; k as usize];
    let mut prefix_sum = vec![0; n + 1];

    // Calculate prefix sum of stones
    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + stones[i];
    }

    fn merge(dp: &mut Vec<Vec<Vec<i32>>>, prefix_sum: &Vec<i32>, i: usize, j: usize, k: i32) -> i32 {
        if dp[k as usize][i][j] != -1 {
            return dp[k as usize][i][j];
        }
        if k == 1 {
            return prefix_sum[j + 1] - prefix_sum[i];
        }

        let mut min_cost = i32::MAX;
        for m in i..j {
            let left = merge(dp, prefix_sum, i, m, k - 1);
            let right = merge(dp, prefix_sum, m + 1, j, 1);
            min_cost = min_cost.min(left + right);
        }

        dp[k as usize][i][j] = min_cost;
        min_cost
    }

    merge(&mut dp, &prefix_sum, 0, n - 1, k)
}
