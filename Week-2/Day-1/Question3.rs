// 264. Ugly Number II

// An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.

// Given an integer n, return the nth ugly number.

 

// Example 1:

// Input: n = 10
// Output: 12
// Explanation: [1, 2, 3, 4, 5, 6, 8, 9, 10, 12] is the sequence of the first 10 ugly numbers.
// Example 2:

// Input: n = 1
// Output: 1
// Explanation: 1 has no prime factors, therefore all of its prime factors are limited to 2, 3, and 5.
 

// Constraints:

// 1 <= n <= 1690

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let primes = [2, 3, 5];
        let mut next_ugly = primes.clone();
        let mut increase = [1, 1, 1];
        let mut arr = vec![1];

        for _ in 1..n {
            let smallest = *next_ugly.iter().min().unwrap();
            arr.push(smallest);

            for i in 0..3 {
                if next_ugly[i] == smallest {
                    increase[i] += 1;
                    next_ugly[i] = primes[i] * arr[increase[i] - 1];
                }
            }
        }

        return *arr.last().unwrap();
    }
}