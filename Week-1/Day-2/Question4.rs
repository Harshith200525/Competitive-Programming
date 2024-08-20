// 44. Wildcard Matching
// Given an input string (s) and a pattern (p), implement wildcard pattern matching with support for '?' and '*' where:

// '?' Matches any single character.
// '*' Matches any sequence of characters (including the empty sequence).
// The matching should cover the entire input string (not partial).

// Input: s = "aa", p = "a"
// Output: false
// Explanation: "a" does not match the entire string "aa".

impl Solution {
    pub fn is_match(st: String, pattern: String) -> bool {
        let mut s = 0;
        let mut p = 0;
        let mut mat = 0;
        let mut star_idx = -1;

        let st_chars: Vec<char> = st.chars().collect();
        let pattern_chars: Vec<char> = pattern.chars().collect();

        while s < st_chars.len(){
            if p < pattern_chars.len() && (pattern_chars[p] == '?' || pattern_chars[p] == st_chars[s]) {
                s += 1;
                p += 1;
            }else if p < pattern_chars.len() && pattern_chars[p] == '*'{
                star_idx = p as i32;
                mat = s;
                p += 1;
            }else if star_idx != -1{
                p = (star_idx + 1) as usize;
                mat += 1;
                s = mat;
            }else{
                return false;
            }
        }

        while p < pattern_chars.len() && pattern_chars[p] == '*'{
            p += 1;
        }

        return p == pattern_chars.len();
    }
}