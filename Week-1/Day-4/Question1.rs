// 16. 3Sum Closest
// Given an integer array nums of length n and an integer target, find three integers in nums such that the sum is closest to target.

// Return the sum of the three integers.

// You may assume that each input would have exactly one solution.

// Input: nums = [-1,2,1,-4], target = 1
// Output: 2
// Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut closest_sum = i32::MAX;

        for i in 0..nums.len() - 2{
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            
            while left < right {
                let current_sum = nums[i] + nums[left] + nums[right];

                if current_sum == target {
                    return current_sum;
                }
                
                if (current_sum - target).abs() < (closest_sum - target).abs() {
                    closest_sum = current_sum;
                }

                if current_sum < target {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        return closest_sum;
    }
}