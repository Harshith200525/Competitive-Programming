// 30. Substring with Concatenation of All Words

// You are given a string s and an array of strings words. All the strings of words are of the same length.

// A concatenated string is a string that exactly contains all the strings of any permutation of words concatenated.

// For example, if words = ["ab","cd","ef"], then "abcdef", "abefcd", "cdabef", "cdefab", "efabcd", and "efcdab" are all concatenated strings. "acdbef" is not a concatenated string because it is not the concatenation of any permutation of words.
// Return an array of the starting indices of all the concatenated substrings in s. You can return the answer in any order.

// Input: s = "barfoothefoobarman", words = ["foo","bar"]

// Output: [0,9]

// Explanation:

// The substring starting at 0 is "barfoo". It is the concatenation of ["bar","foo"] which is a permutation of words.
// The substring starting at 9 is "foobar". It is the concatenation of ["foo","bar"] which is a permutation of words.

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut ret = vec![];
        let n = s.len();
        let k = words.len();
        let m = words[0].len();
        let p = m * k;
        let mut init_window = VecDeque::new();
        let mut not_valid = HashSet::new();
        let mut valid = HashSet::new();

        if n < p {
            return vec![];
        }

        fn valid_substring(substring: &str, words: &Vec<String>, m: usize, p: usize) -> bool {
            let mut temp_words = words.clone();
            let mut i = 0;
            while i < p {
                let curr_word = &substring[i..i + m];
                if let Some(pos) = temp_words.iter().position(|w| w == curr_word) {
                    temp_words.remove(pos);
                }
                i += m;
            }
            return temp_words.is_empty();
        }

        for i in 0..p {
            init_window.push_back(s.chars().nth(i).unwrap());
        }

        if valid_substring(&init_window.iter().collect::<String>(), &words, m, p) {
            ret.push(0);
        }

        let mut i = p;

        while i < n {
            let letter = s.chars().nth(i).unwrap();
            init_window.pop_front();
            init_window.push_back(letter);
            let temp_string = init_window.iter().collect::<String>();

            if !not_valid.contains(&temp_string) {
                if valid.contains(&temp_string) || valid_substring(&temp_string, &words, m, p) {
                    ret.push((i - p + 1) as i32);
                    valid.insert(temp_string.clone());
                } else {
                    not_valid.insert(temp_string.clone());
                }
            }

            i += 1;
        }

        return ret;
    }
}