// Question 4: Split Array into Consecutive Subsequences
// How to determine if an array can be split into consecutive subsequences using greedy algorithms?

use std::collections::HashMap;

fn question_4(nums: Vec<i32>) -> bool {
    let mut count = HashMap::new();
    let mut tails = HashMap::new();

    for &num in &nums {
        *count.entry(num).or_insert(0) += 1;
    }

    for &num in &nums {
        if *count.get(&num).unwrap_or(&0) == 0 {
            continue;
        }
        *count.get_mut(&num).unwrap() -= 1;

        if *tails.get(&(num - 1)).unwrap_or(&0) > 0 {
            *tails.get_mut(&(num - 1)).unwrap() -= 1;
            *tails.entry(num).or_insert(0) += 1;
        } else if *count.get(&(num + 1)).unwrap_or(&0) > 0 && *count.get(&(num + 2)).unwrap_or(&0) > 0 {
            *count.get_mut(&(num + 1)).unwrap() -= 1;
            *count.get_mut(&(num + 2)).unwrap() -= 1;
            *tails.entry(num + 2).or_insert(0) += 1;
        } else {
            return false;
        }
    }

    true
}
