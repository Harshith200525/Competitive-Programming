// 564. Find the Closest Palindrome

// Given a string n representing an integer, return the closest integer (not including itself), which is a palindrome. If there is a tie, return the smaller one.

// The closest is defined as the absolute difference minimized between two integers.

 

// Example 1:

// Input: n = "123"
// Output: "121"
// Example 2:

// Input: n = "1"
// Output: "0"
// Explanation: 0 and 2 are the closest palindromes but we return the smallest which is 0.
 

// Constraints:

// 1 <= n.length <= 18
// n consists of only digits.
// n does not have leading zeros.
// n is representing an integer in the range [1, 1018 - 1]

impl Solution {
    pub fn nearest_palindromic(number_str: String) -> String {
        let number: i64 = number_str.parse().unwrap();
        if number <= 10 {
            return (number - 1).to_string();
        }
        if number == 11 {
            return "9".to_string();
        }

        let length = number_str.len();
        let left_half: i64 = number_str[..(length + 1) / 2].parse().unwrap();
        
        let mut palindrome_candidates = vec![
            Self::generate_palindrome_from_left(left_half - 1, length % 2 == 0),
            Self::generate_palindrome_from_left(left_half, length % 2 == 0),
            Self::generate_palindrome_from_left(left_half + 1, length % 2 == 0),
            10i64.pow((length - 1) as u32) - 1,
            10i64.pow(length as u32) + 1
        ];

        let mut nearest_palindrome = 0;
        let mut min_difference = i64::MAX;

        for candidate in palindrome_candidates.iter() {
            if *candidate == number {
                continue;
            }
            let difference = (candidate - number).abs();
            if difference < min_difference || (difference == min_difference && *candidate < nearest_palindrome) {
                min_difference = difference;
                nearest_palindrome = *candidate;
            }
        }

        nearest_palindrome.to_string()
    }

    fn generate_palindrome_from_left(mut left_half: i64, is_even_length: bool) -> i64 {
        let mut palindrome = left_half;
        if !is_even_length {
            left_half /= 10;
        }
        while left_half > 0 {
            palindrome = palindrome * 10 + left_half % 10;
            left_half /= 10;
        }
        palindrome
    }
}