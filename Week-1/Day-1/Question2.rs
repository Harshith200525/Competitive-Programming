//Question
// Wordle
// Chef invented a modified wordle.

// There is a hidden word S and a guess word T, both of length 5.

// Chef defines a string M to determine the correctness of the guess word. For the ith index:

// If the guess at the ith index is correct, the ith character of M is G.
// If the guess at the ith index is wrong, the ith character of M is B.
// Given the hidden word S and guess T, determine string M.

//Answer
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let s = lines.next().unwrap().unwrap();
        
        let t = lines.next().unwrap().unwrap();
        
        let result = generate_result(&s, &t);
        
        println!("{}", result);
    }
}

fn generate_result(s: &str, t: &str) -> String {
    let mut m = String::new();
    
    // Implement your solution logic here
    for i in 0..s.len(){
        if &s.chars().nth(i) == &t.chars().nth(i){
            m.push('G');
        }else{
            m.push('B');
        }
    }
    return m;
}