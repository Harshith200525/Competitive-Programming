// Question 3: Burst Balloons
// How to determine the maximum coins obtainable by bursting balloons?

fn question_3(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut nums = vec![1]
        .into_iter()
        .chain(nums.into_iter())
        .chain(vec![1].into_iter())
        .collect::<Vec<i32>>();
    let mut dp = vec![vec![0; n + 2]; n + 2];

    for length in 1..=n {
        for left in 1..=n - length + 1 {
            let right = left + length - 1;

            for i in left..=right {
                dp[left][right] = dp[left][right].max(
                    dp[left][i - 1] + nums[left - 1] * nums[i] * nums[right + 1] + dp[i + 1][right],
                );
            }
        }
    }

    dp[1][n]
}
