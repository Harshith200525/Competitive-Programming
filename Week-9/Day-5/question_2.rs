// Question 2: Sliding Window Maximum
// How to find the maximum value in each sliding window of a given size?

use std::collections::VecDeque;

fn question_2(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut result = Vec::new();
    let mut deque = VecDeque::new();

    for i in 0..nums.len() {
        while let Some(&front) = deque.front() {
            if front < i as i32 + 1 - k as i32 {
                deque.pop_front();
            } else {
                break;
            }
        }

        while let Some(&back) = deque.back() {
            if nums[back as usize] <= nums[i] {
                deque.pop_back();
            } else {
                break;
            }
        }

        deque.push_back(i as i32);

        if i >= k - 1 {
            result.push(nums[*deque.front().unwrap() as usize]);
        }
    }

    result
}
