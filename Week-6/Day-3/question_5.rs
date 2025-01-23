// Question 5: Word Ladder II
// How to find all transformation sequences between two words with the minimum number of edits using BFS in Rust?

fn question_5(start: String, end: String, word_list: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::{HashSet, VecDeque};

    let word_set: HashSet<_> = word_list.into_iter().collect();
    let mut results = vec![];
    let mut queue = VecDeque::new();
    queue.push_back(vec![start.clone()]);

    while let Some(path) = queue.pop_front() {
        if let Some(last_word) = path.last() {
            if last_word == &end {
                results.push(path.clone());
            }
            for i in 0..last_word.len() {
                let mut chars: Vec<char> = last_word.chars().collect();
                for c in 'a'..='z' {
                    chars[i] = c;
                    let next_word: String = chars.iter().collect();
                    if word_set.contains(&next_word) && !path.contains(&next_word) {
                        let mut new_path = path.clone();
                        new_path.push(next_word.clone());
                        queue.push_back(new_path);
                    }
                }
            }
        }
    }
    results
}
