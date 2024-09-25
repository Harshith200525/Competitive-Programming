// 204. Count Primes

// Given an integer n, return the number of prime numbers that are strictly less than n.

 

// Example 1:

// Input: n = 10
// Output: 4
// Explanation: There are 4 prime numbers less than 10, they are 2, 3, 5, 7.
// Example 2:

// Input: n = 0
// Output: 0
// Example 3:

// Input: n = 1
// Output: 0
 

// Constraints:

// 0 <= n <= 5 * 106

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }

        let mut seen = vec![false; n as usize];
        let mut ans = 0;

        for num in 2..n {
            if seen[num as usize] {
                continue;
            }
            ans += 1;
            let mut mult = num * num;
            while mult < n {
                seen[mult as usize] = true;
                mult += num;
            }
        }

        ans
    }
}
