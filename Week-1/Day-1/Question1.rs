// Question
// DNA Storage
// For encoding an even-length binary string into a sequence of A, T, C, and G, we iterate from left to right and replace the characters as follows:

// 00 is replaced with A
// 01 is replaced with T
// 10 is replaced with C
// 11 is replaced with G

// Answer

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: u32 = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let n: usize = input.trim().parse().expect("Invalid input");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let s: String = input.trim().to_string();

        // Process the test case with N and S
        // ...
        let mut output = String::new();
        for i in (0..n).step_by(2){
            let slice = &s[i..i+2];
            match slice{
                "00" => output.push('A'),
                "01" => output.push('T'),
                "10" => output.push('C'),
                _ => output.push('G'),
            }
            
        }
        println!("{}",output);
    }
}