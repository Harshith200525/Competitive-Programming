// 76. Minimum Window Substring

// Given two strings s and t of lengths m and n respectively, return the minimum window 
// substring
//  of s such that every character in t (including duplicates) is included in the window. If there is no such substring, return the empty string "".

// The testcases will be generated such that the answer is unique.

 

// Example 1:

// Input: s = "ADOBECODEBANC", t = "ABC"
// Output: "BANC"
// Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C' from string t.
// Example 2:

// Input: s = "a", t = "a"
// Output: "a"
// Explanation: The entire string s is the minimum window.
// Example 3:

// Input: s = "a", t = "aa"
// Output: ""
// Explanation: Both 'a's from t must be included in the window.
// Since the largest window of s only has one 'a', return empty string.
 

// Constraints:

// m == s.length
// n == t.length
// 1 <= m, n <= 105
// s and t consist of uppercase and lowercase English letters.

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return String::new();
        }

        let mut char_count: HashMap<char, i32> = HashMap::new();
        for ch in t.chars() {
            *char_count.entry(ch).or_insert(0) += 1;
        }

        let mut target_chars_remaining = t.len() as i32;
        let mut min_window = (0, i32::MAX);
        let mut start_index = 0;

        for (end_index, ch) in s.chars().enumerate() {
            if let Some(&count) = char_count.get(&ch) {
                if count > 0 {
                    target_chars_remaining -= 1;
                }
                *char_count.entry(ch).or_insert(0) -= 1;
            }

            while target_chars_remaining == 0 {
                let char_at_start = s.chars().nth(start_index).unwrap();
                if let Some(&count) = char_count.get(&char_at_start) {
                    if count == 0 {
                        break;
                    }
                }

                *char_count.entry(char_at_start).or_insert(0) += 1;
                start_index += 1;

                // Update the minimum window if needed
                if end_index as i32 - start_index as i32 < min_window.1 - min_window.0 {
                    min_window = (start_index as i32, end_index as i32);
                }
            }
            
            // Reset after finding a valid window
            if let Some(count) = char_count.get_mut(&s.chars().nth(start_index).unwrap()) {
                *count += 1;
            }
            target_chars_remaining += 1;
        }

        if min_window.1 > s.len() as i32 {
            return String::new();
        }
        
        s[min_window.0 as usize..=min_window.1 as usize].to_string()
    }
}