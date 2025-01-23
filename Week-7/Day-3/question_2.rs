// Question 2: Find All Anagrams in a String
// How to find all start indices of an anagram of a given string in another string using Rust?

use std::collections::HashMap;

fn question_2(s: String, p: String) -> Vec<i32> {
    let mut result = Vec::new();
    let mut p_count = HashMap::new();
    let mut s_count = HashMap::new();

    for c in p.chars() {
        *p_count.entry(c).or_insert(0) += 1;
    }

    let mut left = 0;
    for right in 0..s.len() {
        let right_char = s.chars().nth(right).unwrap();
        *s_count.entry(right_char).or_insert(0) += 1;

        if right - left + 1 > p.len() {
            let left_char = s.chars().nth(left).unwrap();
            *s_count.entry(left_char).or_insert(0) -= 1;
            if s_count[&left_char] == 0 {
                s_count.remove(&left_char);
            }
            left += 1;
        }

        if s_count == p_count {
            result.push(left as i32);
        }
    }

    result
}
