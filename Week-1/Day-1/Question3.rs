// Different Consecutive Characters
// Chef has a binary string S of length N. Chef can perform the following operation on S:

// Insert any character (0 or 1) at any position in S. Find the minimum number of operations Chef needs to perform so that no two consecutive characters are same in S.

// Input
// 3
// 2
// 11
// 4
// 0101
// 5
// 00100

// Output
// 1
// 0
// 2

use std::io::{self, BufRead};

fn process_case(s: String) {
    // Implement your solution logic here
    let n: usize = s.len();
    let mut output: usize = 0;
    for i in 0..n-1{
        if &s.chars().nth(i) == &s.chars().nth(i+1){
            output += 1;
        }
    }
    println!("{}", output);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let _n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        
        let s = lines.next().unwrap().unwrap().trim().to_string();
        
        process_case(s);
    }
}