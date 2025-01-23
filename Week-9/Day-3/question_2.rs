// Question 2: Maximum Erasure Value
// How to find the maximum sum of a subarray with unique elements using a sliding window approach?

use std::collections::HashSet;

fn question_2(nums: Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    let mut left = 0;
    let mut max_sum = 0;
    let mut current_sum = 0;

    for right in 0..nums.len() {
        while set.contains(&nums[right]) {
            set.remove(&nums[left]);
            current_sum -= nums[left];
            left += 1;
        }

        set.insert(nums[right]);
        current_sum += nums[right];
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}
