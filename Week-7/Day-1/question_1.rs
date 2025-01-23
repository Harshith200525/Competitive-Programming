// Question 1: Word Ladder
// How to find the shortest transformation sequence from one word to another by changing one letter at a time using Rust?

use std::collections::{HashSet, VecDeque};

fn question_1(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let word_set: HashSet<_> = word_list.into_iter().collect();
    if !word_set.contains(&end_word) {
        return 0;
    }

    let mut queue = VecDeque::new();
    queue.push_back((begin_word, 1));

    while let Some((word, steps)) = queue.pop_front() {
        if word == end_word {
            return steps;
        }

        for i in 0..word.len() {
            let mut chars: Vec<char> = word.chars().collect();
            for c in 'a'..='z' {
                let original = chars[i];
                chars[i] = c;
                let next_word: String = chars.iter().collect();
                if word_set.contains(&next_word) {
                    queue.push_back((next_word.clone(), steps + 1));
                }
                chars[i] = original;
            }
        }
    }

    0
}
