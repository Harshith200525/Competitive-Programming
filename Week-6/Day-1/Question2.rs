// 128. Longest Consecutive Sequence

// Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.

// You must write an algorithm that runs in O(n) time.

 

// Example 1:

// Input: nums = [100,4,200,1,3,2]
// Output: 4
// Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
// Example 2:

// Input: nums = [0,3,7,2,5,8,4,6,0,1]
// Output: 9
 

// Constraints:

// 0 <= nums.length <= 105
// -109 <= nums[i] <= 109

use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let num_set: HashSet<i32> = nums.iter().cloned().collect();
        let mut longest = 0;

        for &n in &nums {
            // Check if `n - 1` is not present, meaning `n` could be the start of a sequence
            if !num_set.contains(&(n - 1)) {
                let mut length = 1;
                // Extend the sequence as long as `n + length` is in the set
                while num_set.contains(&(n + length)) {
                    length += 1;
                }
                // Update the longest sequence found
                longest = longest.max(length);
            }
        }

        longest
    }
}
