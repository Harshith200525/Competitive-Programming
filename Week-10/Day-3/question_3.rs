// Question 3: Word Ladder II
// How to find all shortest transformation sequences between words?

use std::collections::{HashSet, VecDeque};

fn question_3(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
    let mut word_set: HashSet<String> = word_list.into_iter().collect();
    if !word_set.contains(&end_word) {
        return vec![];
    }

    let mut queue = VecDeque::new();
    queue.push_back(vec![begin_word.clone()]);
    let mut result = Vec::new();
    let mut level_found = false;

    while !queue.is_empty() && !level_found {
        let mut local_visited = HashSet::new();
        for _ in 0..queue.len() {
            let path = queue.pop_front().unwrap();
            let last_word = path.last().unwrap();

            for i in 0..last_word.len() {
                let mut chars: Vec<char> = last_word.chars().collect();
                for c in 'a'..='z' {
                    let original_char = chars[i];
                    chars[i] = c;
                    let next_word: String = chars.iter().collect();

                    if word_set.contains(&next_word) {
                        let mut new_path = path.clone();
                        new_path.push(next_word.clone());

                        if next_word == end_word {
                            result.push(new_path);
                            level_found = true;
                        } else {
                            queue.push_back(new_path);
                        }
                        local_visited.insert(next_word);
                    }
                    chars[i] = original_char;
                }
            }
        }
        for word in local_visited {
            word_set.remove(&word);
        }
    }

    result
}
