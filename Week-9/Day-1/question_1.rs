// Question 1: Split Array into Consecutive Subsequences
// How to split an array into as many subsequences as possible, each being consecutive and having a length of at least 3?

use std::collections::HashMap;

fn question_1(nums: Vec<i32>) -> bool {
    let mut freq = HashMap::new();
    let mut tails = HashMap::new();

    for &num in &nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    for &num in &nums {
        if freq.get(&num) == Some(&0) {
            continue;
        }

        if let Some(&prev_tail) = tails.get(&(num - 1)) {
            if *freq.entry(num).or_insert(0) > 0 {
                *freq.entry(num).or_insert(0) -= 1;
                *tails.entry(num).or_insert(0) += 1;
                *tails.entry(num + 1).or_insert(0) += 1;
            }
        } else {
            if *freq.entry(num).or_insert(0) > 0 {
                *freq.entry(num).or_insert(0) -= 1;
                *tails.entry(num + 1).or_insert(0) += 1;
            }
        }
    }

    true
}
