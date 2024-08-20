// 719. Find K-th Smallest Pair Distance
// The distance of a pair of integers a and b is defined as the absolute difference between a and b.

// Given an integer array nums and an integer k, return the kth smallest distance among all the pairs nums[i] and nums[j] where 0 <= i < j < nums.length.

// Input: nums = [1,3,1], k = 1
// Output: 0
// Explanation: Here are all the pairs:
// (1,3) -> 2
// (1,1) -> 0
// (3,1) -> 2
// Then the 1st smallest distance pair is (1,1), and its distance is 0.

impl Solution {
    fn count_pairs(nums: &Vec<i32>, distance: i32) -> i32 {
        let mut count = 0;
        let mut left = 0;

        for right in 1..nums.len() {
            while nums[right] - nums[left] > distance {
                left += 1;
            }
            count += (right - left) as i32;
        }

        return count;
    }

    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut low = 0;
        let mut high = nums[nums.len() - 1] - nums[0];

        while low < high {
            let mid = (low + high) / 2;
            if Self::count_pairs(&nums, mid) < k{
                low = mid + 1;
            }else{
                high = mid;
            }
        }

        return low;
    }
}