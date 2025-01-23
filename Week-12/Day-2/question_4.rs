// Question 4: Palindrome Pairs
// How to find all pairs of words that can form palindromes when concatenated using hash maps?

use std::collections::HashMap;

fn question_4(words: Vec<String>) -> Vec<Vec<i32>> {
    let mut word_map = HashMap::new();
    for (i, word) in words.iter().enumerate() {
        word_map.insert(word.chars().rev().collect::<String>(), i as i32);
    }

    let mut result = Vec::new();

    for (i, word) in words.iter().enumerate() {
        for j in 0..=word.len() {
            let prefix = &word[..j];
            let suffix = &word[j..];

            if is_palindrome(suffix) {
                if let Some(&idx) = word_map.get(prefix) {
                    if idx != i as i32 {
                        result.push(vec![i as i32, idx]);
                    }
                }
            }

            if !prefix.is_empty() && is_palindrome(prefix) {
                if let Some(&idx) = word_map.get(suffix) {
                    if idx != i as i32 {
                        result.push(vec![idx, i as i32]);
                    }
                }
            }
        }
    }

    result
}

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}
