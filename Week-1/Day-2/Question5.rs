// Regular Expression Matching
// Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:

// '.' Matches any single character.​​​​
// '*' Matches zero or more of the preceding element.
// The matching should cover the entire input string (not partial).

// Input: s = "aa", p = "a"
// Output: false
// Explanation: "a" does not match the entire string "aa".

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s_len = s.len();
        let p_len = p.len();

        let mut mat = vec![vec![false; p_len+1]; s_len+1];
        mat[0][0] = true;

        let s_chars: Vec<char> = s.chars().collect();
        let p_chars: Vec<char> = p.chars().collect();

        for i in 1..=p_len {
            if p_chars[i - 1] == '*'{
                mat[0][i] = mat[0][i-2];
            }
        }

        for i in 1..=s_len {
        for j in 1..=p_len {
            if p_chars[j - 1] == '.' || p_chars[j - 1] == s_chars[i - 1] {
                mat[i][j] = mat[i - 1][j - 1];
            } else if p_chars[j - 1] == '*' {
                mat[i][j] = mat[i][j - 2];
                if j >= 2 && (p_chars[j - 2] == '.' || p_chars[j - 2] == s_chars[i - 1]) {
                    mat[i][j] = mat[i][j] || mat[i - 1][j];
                }
            } else {
                mat[i][j] = false;
            }
        }
    }

    return mat[s_len][p_len]
    }
}