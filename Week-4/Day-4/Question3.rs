// 227. Basic Calculator II

// Given a string s which represents an expression, evaluate this expression and return its value. 

// The integer division should truncate toward zero.

// You may assume that the given expression is always valid. All intermediate results will be in the range of [-231, 231 - 1].

// Note: You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as eval().

 

// Example 1:

// Input: s = "3+2*2"
// Output: 7
// Example 2:

// Input: s = " 3/2 "
// Output: 1
// Example 3:

// Input: s = " 3+5 / 2 "
// Output: 5
 

// Constraints:

// 1 <= s.length <= 3 * 105
// s consists of integers and operators ('+', '-', '*', '/') separated by some number of spaces.
// s represents a valid expression.
// All the integers in the expression are non-negative integers in the range [0, 231 - 1].
// The answer is guaranteed to fit in a 32-bit integer.

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        let mut num = 0;
        let mut prev_operator = '+';
        let s = s.chars().chain(std::iter::once('\0')); // Append a terminator

        for ch in s {
            if ch.is_digit(10) {
                num = num * 10 + ch.to_digit(10).unwrap() as i32;
            } else if !ch.is_digit(10) && ch != ' ' || ch == '\0' {
                match prev_operator {
                    '+' => stack.push(num),
                    '-' => stack.push(-num),
                    '*' => {
                        if let Some(top) = stack.pop() {
                            stack.push(top * num);
                        }
                    }
                    '/' => {
                        if let Some(top) = stack.pop() {
                            stack.push((top as f32 / num as f32).floor() as i32);
                        }
                    }
                    _ => {}
                }

                prev_operator = ch;
                num = 0;
            }
        }

        stack.iter().sum()
    }
}
