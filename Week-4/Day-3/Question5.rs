// 202. Happy Number

// Write an algorithm to determine if a number n is happy.

// A happy number is a number defined by the following process:

// Starting with any positive integer, replace the number by the sum of the squares of its digits.
// Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
// Those numbers for which this process ends in 1 are happy.
// Return true if n is a happy number, and false if not.

 

// Example 1:

// Input: n = 19
// Output: true
// Explanation:
// 12 + 92 = 82
// 82 + 22 = 68
// 62 + 82 = 100
// 12 + 02 + 02 = 1
// Example 2:

// Input: n = 2
// Output: false
 

// Constraints:

// 1 <= n <= 231 - 1

use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut visited = HashSet::new();
        let mut current = n;

        // Function to calculate the next number in the sequence
        fn get_next_number(n: i32) -> i32 {
            let mut num = n;
            let mut sum = 0;
            while num > 0 {
                let digit = num % 10;
                sum += digit * digit;
                num /= 10;
            }
            sum
        }

        while !visited.contains(&current) {
            visited.insert(current);
            current = get_next_number(current);
            if current == 1 {
                return true;
            }
        }

        false
    }
}
