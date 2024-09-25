// 238. Product of Array Except Self

// Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].

// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

// You must write an algorithm that runs in O(n) time and without using the division operation.

 

// Example 1:

// Input: nums = [1,2,3,4]
// Output: [24,12,8,6]
// Example 2:

// Input: nums = [-1,1,0,-3,3]
// Output: [0,0,9,0,0]
 

// Constraints:

// 2 <= nums.length <= 105
// -30 <= nums[i] <= 30
// The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut left = vec![1; n];
        let mut right = vec![1; n];
        let mut result = vec![0; n];

        // Build the `left` array
        for i in 1..n {
            left[i] = left[i - 1] * nums[i - 1];
        }

        // Build the `right` array
        for i in (0..n-1).rev() {
            right[i] = right[i + 1] * nums[i + 1];
        }

        // Combine the `left` and `right` arrays to form the result
        for i in 0..n {
            result[i] = left[i] * right[i];
        }

        result
    }
}
