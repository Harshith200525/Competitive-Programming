// Question 2: Substring with Concatenation of All Words
// How to find all starting indices of substrings formed by concatenating all words from a list?

use std::collections::HashMap;

fn question_2(s: String, words: Vec<String>) -> Vec<i32> {
    if words.is_empty() {
        return vec![];
    }

    let word_len = words[0].len();
    let word_count = words.len();
    let substr_len = word_len * word_count;
    let mut result = Vec::new();
    let word_map: HashMap<_, _> = words.iter().cloned().fold(HashMap::new(), |mut acc, word| {
        *acc.entry(word).or_insert(0) += 1;
        acc
    });

    for i in 0..word_len {
        let mut left = i;
        let mut count = 0;
        let mut seen = HashMap::new();

        for right in (i..=s.len() - word_len).step_by(word_len) {
            let word = &s[right..right + word_len];
            if let Some(&count_in_words) = word_map.get(word) {
                *seen.entry(word).or_insert(0) += 1;

                if seen[word] <= count_in_words {
                    count += 1;
                } else {
                    while seen[word] > count_in_words {
                        let left_word = &s[left..left + word_len];
                        *seen.entry(left_word).or_insert(0) -= 1;
                        if seen[left_word] < *word_map.get(left_word).unwrap_or(&0) {
                            count -= 1;
                        }
                        left += word_len;
                    }
                }

                if count == word_count {
                    result.push(left as i32);
                    let left_word = &s[left..left + word_len];
                    *seen.entry(left_word).or_insert(0) -= 1;
                    count -= 1;
                    left += word_len;
                }
            } else {
                seen.clear();
                count = 0;
                left = right + word_len;
            }
        }
    }

    result
}
