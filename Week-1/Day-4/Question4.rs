// 45. Jump Game II

// You are given a 0-indexed array of integers nums of length n. You are initially positioned at nums[0].

// Each element nums[i] represents the maximum length of a forward jump from index i. In other words, if you are at nums[i], you can jump to any nums[i + j] where:

// 0 <= j <= nums[i] and
// i + j < n
// Return the minimum number of jumps to reach nums[n - 1]. The test cases are generated such that you can reach nums[n - 1].

// Input: nums = [2,3,1,1,4]
// Output: 2
// Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index.

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut near = 0;
        let mut far = 0;
        let mut jumps = 0;

        let n = nums.len();

        while far < n-1 {
            let mut farthest = 0;

            for i in near..=far {
                farthest = farthest.max(i + nums[i] as usize);
            }

            near = far + 1;
            far = farthest;
            jumps += 1;
        }

        return jumps;
    }
}