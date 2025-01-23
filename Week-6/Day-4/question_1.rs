// Question 1: Jump Game II
// How to find the minimum number of jumps needed to reach the end of an array using Rust?

fn question_1(nums: Vec<i32>) -> i32 {
    let mut jumps = 0;
    let mut curr_end = 0;
    let mut farthest = 0;

    for i in 0..nums.len() - 1 {
        farthest = farthest.max(i + nums[i] as usize);
        if i == curr_end {
            jumps += 1;
            curr_end = farthest;
        }
    }

    jumps
}
