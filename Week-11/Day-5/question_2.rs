// Question 2: Find Minimum in Rotated Sorted Array II
// How to find the minimum in a rotated sorted array with duplicates using binary search?

fn question_2(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] < nums[right] {
            right = mid;
        } else if nums[mid] > nums[right] {
            left = mid + 1;
        } else {
            right -= 1;
        }
    }

    nums[left]
}
