// Question 4: Concatenated Words
// How to find words that are formed by concatenating other words in a list using a Trie and recursive backtracking?

use std::collections::HashSet;

fn question_4(words: Vec<String>) -> Vec<String> {
    let mut word_set: HashSet<String> = words.into_iter().collect();
    let mut result = Vec::new();

    fn can_form(word: &str, word_set: &HashSet<String>, start: usize, count: usize) -> bool {
        if start == word.len() {
            return count >= 2;
        }

        for i in start + 1..=word.len() {
            let prefix = &word[start..i];
            if word_set.contains(prefix) && can_form(word, word_set, i, count + 1) {
                return true;
            }
        }

        false
    }

    for word in word_set.clone() {
        if can_form(&word, &word_set, 0, 0) {
            result.push(word);
        }
    }

    result
}
