// 17. Letter Combinations of a Phone Number

// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.

// A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.


 

// Example 1:

// Input: digits = "23"
// Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
// Example 2:

// Input: digits = ""
// Output: []
// Example 3:

// Input: digits = "2"
// Output: ["a","b","c"]
 

// Constraints:

// 0 <= digits.length <= 4
// digits[i] is a digit in the range ['2', '9'].

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        // Define a mapping of digits to letters
        let digit_to_letters = vec![
            "".to_string(), // 0
            "".to_string(), // 1
            "abc".to_string(),
            "def".to_string(),
            "ghi".to_string(),
            "jkl".to_string(),
            "mno".to_string(),
            "pqrs".to_string(),
            "tuv".to_string(),
            "wxyz".to_string(),
        ];
        
        let mut res = Vec::new();
        if digits.is_empty() {
            return res;
        }
        
        fn backtrack(
            digits: &str,
            idx: usize,
            comb: String,
            digit_to_letters: &[String],
            res: &mut Vec<String>
        ) {
            if idx == digits.len() {
                res.push(comb);
                return;
            }
            
            let digit = digits[idx..idx + 1].parse::<usize>().unwrap();
            let letters = &digit_to_letters[digit];
            
            for letter in letters.chars() {
                let mut new_comb = comb.clone();
                new_comb.push(letter);
                backtrack(digits, idx + 1, new_comb, digit_to_letters, res);
            }
        }
        
        backtrack(&digits, 0, String::new(), &digit_to_letters, &mut res);
        res
    }
}
