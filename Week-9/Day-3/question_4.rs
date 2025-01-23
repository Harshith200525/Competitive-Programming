// Question 4: Find Kth Smallest Pair Distance
// How to find the k-th smallest distance between pairs in a sorted array?

fn question_4(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort();
    
    let mut left = 0;
    let mut right = nums[nums.len() - 1] - nums[0];

    while left < right {
        let mid = left + (right - left) / 2;
        let mut count = 0;
        let mut j = 0;

        for i in 0..nums.len() {
            while j < nums.len() && nums[j] - nums[i] <= mid {
                j += 1;
            }
            count += j - i - 1;
        }

        if count >= k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}
