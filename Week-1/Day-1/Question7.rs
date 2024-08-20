// Problem Statement: Chef's Happiness
// Chef has a string S with him. Chef is happy if the string contains a contiguous substring of length strictly greater than 2 in which all its characters are vowels.

// Determine whether Chef is happy or not.

// Note: In the English alphabet, vowels are a, e, i, o, and u.

// Input
// 4
// aeiou
// abxy
// aebcdefghij
// abcdeeafg

// Output
// Happy
// Sad
// Sad
// Happy

use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut first_line = String::new();
    handle.read_line(&mut first_line).expect("Failed to read line");
    let t: usize = first_line.trim().parse().expect("Input was not a number");

    for _ in 0..t {
        let mut line = String::new();
        handle.read_line(&mut line).expect("Failed to read string");
        let s = line.trim().to_string();
        
        // Call your function to process each case here
        process_case(&s);
    }
}

fn process_case(s: &str) {
    // This function should contain your logic to determine if Chef is happy or sad.
    // Implement your solution here and print the result.
    let vow: HashSet<char> = ['a', 'e', 'i', 'o', 'u'].iter().cloned().collect();
    let mut chef = "Sad";
    
    for i in 0..s.len() - 2{
        if vow.contains(&s.chars().nth(i).unwrap()) && 
        vow.contains(&s.chars().nth(i+1).unwrap()) &&
        vow.contains(&s.chars().nth(i+2).unwrap()){
            chef = "Happy";
            break;
        }
    }
    
    println!("{}", chef)
}