// 7. Reverse Integer

// Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.

// Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

// Input: x = 123
// Output: 321

impl Solution {
    pub fn reverse(x: i32) -> i32 {
       let mut res: i64 = 0;
       let mut num = x;

       while num != 0{
        res = res * 10 + (num % 10) as i64;
        num /= 10;
       }

       if res > i32::MAX as i64 || res < i32::MIN as i64 {
        return 0;
       }

       return res as i32;
    }
}