// Question 3: Word Ladder II
// How to find all shortest transformation sequences from one word to another, changing only one letter at a time?

use std::collections::{HashSet, VecDeque};

fn question_3(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
    let mut word_set: HashSet<String> = word_list.into_iter().collect();
    let mut result = Vec::new();
    if !word_set.contains(&end_word) {
        return result;
    }

    let mut queue = VecDeque::new();
    queue.push_back(vec![begin_word.clone()]);

    let mut visited = HashSet::new();
    visited.insert(begin_word.clone());

    while !queue.is_empty() {
        let mut local_visited = HashSet::new();
        let mut level = queue.len();
        let mut found = false;

        for _ in 0..level {
            let path = queue.pop_front().unwrap();
            let word = path.last().unwrap().clone();

            for i in 0..word.len() {
                let mut chars: Vec<char> = word.chars().collect();
                for c in 'a'..='z' {
                    chars[i] = c;
                    let next_word: String = chars.iter().collect();
                    if word_set.contains(&next_word) && !visited.contains(&next_word) {
                        let mut new_path = path.clone();
                        new_path.push(next_word.clone());
                        if next_word == end_word {
                            result.push(new_path);
                            found = true;
                        } else {
                            local_visited.insert(next_word.clone());
                            queue.push_back(new_path);
                        }
                    }
                }
            }
        }

        if found {
            break;
        }

        for word in local_visited {
            visited.insert(word);
        }
    }

    result
}
