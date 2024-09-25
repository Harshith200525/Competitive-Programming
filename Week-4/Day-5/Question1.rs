// 46. Permutations

// Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.

 

// Example 1:

// Input: nums = [1,2,3]
// Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
// Example 2:

// Input: nums = [0,1]
// Output: [[0,1],[1,0]]
// Example 3:

// Input: nums = [1]
// Output: [[1]]
 

// Constraints:

// 1 <= nums.length <= 6
// -10 <= nums[i] <= 10
// All the integers of nums are unique.

pub struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut res = Vec::new();
        Self::permute_helper(&mut nums, 0, &mut res);
        res
    }

    fn permute_helper(nums: &mut Vec<i32>, start: usize, res: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            res.push(nums.clone());
            return;
        }

        for i in start..nums.len() {
            nums.swap(start, i); // Swap the current index with the start
            Self::permute_helper(nums, start + 1, res); // Recur for the next index
            nums.swap(start, i); // Backtrack to the original list
        }
    }
}
