// Question 2: Word Break II
// How to find all possible sentences from a string using a dictionary of words?

use std::collections::HashMap;

fn question_2(s: String, word_dict: Vec<String>) -> Vec<String> {
    let mut memo = HashMap::new();
    let word_set: std::collections::HashSet<_> = word_dict.into_iter().collect();

    fn backtrack(s: &str, word_set: &std::collections::HashSet<String>, memo: &mut HashMap<String, Vec<String>>) -> Vec<String> {
        if let Some(result) = memo.get(s) {
            return result.clone();
        }

        let mut sentences = Vec::new();
        if s.is_empty() {
            sentences.push("".to_string());
            return sentences;
        }

        for i in 1..=s.len() {
            let prefix = &s[0..i];
            if word_set.contains(prefix) {
                let suffix = &s[i..];
                let suffix_sentences = backtrack(suffix, word_set, memo);

                for sentence in suffix_sentences {
                    if sentence.is_empty() {
                        sentences.push(prefix.to_string());
                    } else {
                        sentences.push(format!("{} {}", prefix, sentence));
                    }
                }
            }
        }

        memo.insert(s.to_string(), sentences.clone());
        sentences
    }

    backtrack(&s, &word_set, &mut memo)
}
