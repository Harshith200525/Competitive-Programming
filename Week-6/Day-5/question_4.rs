// Question 4: Remove Invalid Parentheses
// How to remove the minimum number of invalid parentheses to make a string valid using Rust?

use std::collections::{HashSet, VecDeque};

fn question_4(s: String) -> Vec<String> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut result = Vec::new();

    queue.push_back(s.clone());
    visited.insert(s);
    let mut found = false;

    while let Some(current) = queue.pop_front() {
        if is_valid(&current) {
            result.push(current.clone());
            found = true;
        }
        if found {
            continue;
        }
        for i in 0..current.len() {
            if !current.chars().nth(i).unwrap().is_alphabetic() {
                let mut next = current.clone();
                next.remove(i);
                if visited.insert(next.clone()) {
                    queue.push_back(next);
                }
            }
        }
    }

    result
}

fn is_valid(s: &str) -> bool {
    let mut balance = 0;
    for ch in s.chars() {
        if ch == '(' {
            balance += 1;
        } else if ch == ')' {
            balance -= 1;
        }
        if balance < 0 {
            return false;
        }
    }
    balance == 0
}
