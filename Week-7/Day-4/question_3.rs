// Question 3: Minimum Window Substring
// How to find the smallest window in a string that contains all characters of another string using Rust?

use std::collections::HashMap;

fn question_3(s: String, t: String) -> String {
    let mut need = HashMap::new();
    for c in t.chars() {
        *need.entry(c).or_insert(0) += 1;
    }

    let mut window = HashMap::new();
    let (mut left, mut right) = (0, 0);
    let (mut start, mut min_len) = (0, usize::MAX);
    let mut valid = 0;

    let s_chars: Vec<char> = s.chars().collect();

    while right < s_chars.len() {
        let c = s_chars[right];
        right += 1;

        if let Some(count) = need.get(&c) {
            let entry = window.entry(c).or_insert(0);
            *entry += 1;
            if *entry == *count {
                valid += 1;
            }
        }

        while valid == need.len() {
            if right - left < min_len {
                start = left;
                min_len = right - left;
            }
            let d = s_chars[left];
            left += 1;

            if let Some(count) = need.get(&d) {
                if let Some(entry) = window.get_mut(&d) {
                    if *entry == *count {
                        valid -= 1;
                    }
                    *entry -= 1;
                }
            }
        }
    }

    if min_len == usize::MAX {
        "".to_string()
    } else {
        s[start..start + min_len].to_string()
    }
}
