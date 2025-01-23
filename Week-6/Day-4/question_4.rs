// Question 4: Find Minimum in Rotated Sorted Array
// How to find the minimum element in a rotated sorted array using Rust?

fn question_4(nums: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0, nums.len() - 1);

    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] > nums[right] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    nums[left]
}
