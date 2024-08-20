// 35. Search Insert Position
// Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

// You must write an algorithm with O(log n) runtime complexity.

// Input: nums = [1,3,5,6], target = 5
// Output: 2

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len() as i32 - 1;

        while l <= r {
            let mid = l + (r - l) / 2;

            if nums[mid as usize] == target{
                return mid;
            }else if nums[mid as usize] > target{
                r = mid - 1;
            }else{
                l = mid + 1;
            }
        }
        return l;
    }
}