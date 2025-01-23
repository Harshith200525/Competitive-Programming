// Question 4: Search in Rotated Sorted Array
// How to search for a target value in a rotated sorted array using Rust?

fn question_4(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, nums.len() as i32 - 1);

    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid as usize] == target {
            return mid;
        }
        if nums[left as usize] <= nums[mid as usize] {
            if target >= nums[left as usize] && target < nums[mid as usize] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            if target > nums[mid as usize] && target <= nums[right as usize] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }

    -1
}
