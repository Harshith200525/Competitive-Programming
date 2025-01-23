// Question 5: Target Sum
// How to find the number of ways to assign + and - to elements in an array to reach a target sum using Rust?

fn question_5(nums: Vec<i32>, target: i32) -> i32 {
    let sum: i32 = nums.iter().sum();
    if target.abs() > sum || (sum + target) % 2 == 1 {
        return 0;
    }
    let new_target = (sum + target) / 2;

    let mut dp = vec![0; (new_target + 1) as usize];
    dp[0] = 1;

    for &num in &nums {
        for j in (num..=new_target).rev() {
            dp[j as usize] += dp[(j - num) as usize];
        }
    }

    dp[new_target as usize]
}
