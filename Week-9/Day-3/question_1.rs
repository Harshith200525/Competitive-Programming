// Question 1: Minimum Operations to Reduce X to Zero
// How to reduce a target value X to zero by removing elements from either end of an array using a sliding window approach?

fn question_1(nums: Vec<i32>, x: i32) -> i32 {
    let total_sum: i32 = nums.iter().sum();
    let target = total_sum - x;
    if target < 0 {
        return -1; // Impossible to reach x
    }

    let mut left = 0;
    let mut current_sum = 0;
    let mut max_len = -1;

    for right in 0..nums.len() {
        current_sum += nums[right];
        
        while current_sum > target {
            current_sum -= nums[left];
            left += 1;
        }

        if current_sum == target {
            max_len = max_len.max(right - left + 1) as i32;
        }
    }

    if max_len == -1 {
        -1 // Not possible
    } else {
        nums.len() as i32 - max_len
    }
}
