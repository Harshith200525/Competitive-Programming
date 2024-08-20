// Convert String to Title Case
// Given a string S consisting of only lowercase and uppercase English letters and spaces, your task is to convert it into title case. In title case, the first letter of each word is capitalized while the rest are in lowercase, except for words that are entirely in uppercase (considered as acronyms), which should remain unchanged.

// Input:
// 5
// hello world
// this is a CODECHEF problem
// WELCOME to the JUNGLE
// the quick BROWN fOx
// programming in PYTHON

// Output:
// Hello World
// This Is A CODECHEF Problem
// WELCOME To The JUNGLE
// The Quick BROWN Fox
// Programming In PYTHON

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let s = lines.next().unwrap().unwrap();
        println!("{}", convert_to_title_case(&s));
    }
}

fn convert_to_title_case(s: &str) -> String {
    // Implement the title case conversion logic here
    let mut output = String::new();
    let st: Vec<&str> = s.trim().split_whitespace().collect(); 
    
    for word in st{
        if word.chars().all(|c| c.is_uppercase()){
            output.push_str(word);
            output.push(' ');
        }else{
            let mut capitalized_word = String::new();
            capitalized_word.push(word.chars().next().unwrap().to_uppercase().next().unwrap());
            capitalized_word.push_str(&word[1..].to_lowercase());
            output.push_str(&capitalized_word);
            output.push(' ');
        }
    }
    
    return output;
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    let first = chars.next().unwrap().to_uppercase().collect::<String>();
    return first + chars.as_str();
}