// Question 2: Arithmetic Slices II - Subsequence
// How to count all arithmetic subsequences in an array?

use std::collections::HashMap;

fn question_2(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![HashMap::new(); n];
    let mut result = 0;

    for i in 1..n {
        for j in 0..i {
            let diff = nums[i] as i64 - nums[j] as i64;
            if diff < i32::MIN as i64 || diff > i32::MAX as i64 {
                continue;
            }
            let diff = diff as i32;

            let count = *dp[j].get(&diff).unwrap_or(&0);
            result += count;

            *dp[i].entry(diff).or_insert(0) += count + 1;
        }
    }

    result
}
