// Question 3: Jump Game VI
// How to find the highest score achievable in a sequence of jumps with restricted scope using dynamic programming and a deque?

use std::collections::VecDeque;

fn question_3(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut dp = vec![0; n];
    dp[0] = nums[0];
    
    let mut dq = VecDeque::new();
    dq.push_back(0);

    for i in 1..n {
        while let Some(&j) = dq.front() {
            if i - j > k as usize {
                dq.pop_front();
            } else {
                break;
            }
        }

        dp[i] = nums[i] + dp[dq.front().copied().unwrap_or(0)];

        while let Some(&j) = dq.back() {
            if dp[j] <= dp[i] {
                dq.pop_back();
            } else {
                break;
            }
        }

        dq.push_back(i);
    }

    *dp.iter().max().unwrap()
}
