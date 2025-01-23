// Question 1: House Robber II
// How to compute the maximum amount that can be robbed from a circular arrangement of houses using dynamic programming in Rust?

fn question_1(nums: Vec<i32>) -> i32 {
    fn rob_linear(houses: &[i32]) -> i32 {
        let (mut prev, mut curr) = (0, 0);
        for &money in houses {
            let temp = curr;
            curr = curr.max(prev + money);
            prev = temp;
        }
        curr
    }
    match nums.len() {
        0 => 0,
        1 => nums[0],
        _ => rob_linear(&nums[..nums.len() - 1]).max(rob_linear(&nums[1..])),
    }
}
