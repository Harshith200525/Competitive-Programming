// 41. First Missing Positive

// Given an unsorted integer array nums. Return the smallest positive integer that is not present in nums.

// You must implement an algorithm that runs in O(n) time and uses O(1) auxiliary space.

// Input: nums = [1,2,0]
// Output: 3
// Explanation: The numbers in the range [1,2] are all in the array.

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let nums_length = nums.len();
        let mut i = 0;

        while i < nums_length {
            let val_at_i = nums[i] as usize - 1;
            let belongs_in_range = (0..nums_length).contains(&val_at_i);

            if belongs_in_range && nums[i] != nums[val_at_i] as i32 {
                nums.swap(i, val_at_i);
            } else {
                i += 1;
            }
        }

        for (index, &num) in nums.iter().enumerate() {
            if num != (index + 1) as i32 {
                return (index + 1) as i32;
            }
        }

        return nums_length as i32 + 1;
    }
}