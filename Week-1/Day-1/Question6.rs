// Add One
// You are given a large number N. You need to print the number N + 1.

// Note: The number is very large, and it will not fit in a standard integer data type. You must take the input as a string and then manipulate the digits to convert it to N + 1.

// Input:
// 6
// 99
// 17
// 1
// 599
// 10000000000000000000
// 549843954323494990404

// Output:
// 100
// 18
// 2
// 600
// 10000000000000000001
// 549843954323494990405

use std::io::{self, BufRead};

fn solve(n: String) -> String {
    // You can write your logic here to increment the number n by 1
    // For now, it just returns the input string as a placeholder
    let mut carry = 1;
    let mut num = n;
    
    for i in (0..num.len()).rev(){
        let mut temp = num[i..i+1].parse::<u32>().unwrap();
        temp += carry;
        
        if temp > 9{
            num.replace_range(i..i+1, "0");
        }else{
            carry = 0;
            num.replace_range(i..i+1, &temp.to_string());
            break;
        }
    }
    
    if carry == 1{
        num = format!("1{}", num);
    }
    
    return num;
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    for _ in 0..t {
        // Read the large number as a string
        let n = lines.next()
            .unwrap()
            .unwrap()
            .trim()
            .to_string();

        // Call the solve function to process the input
        let result = solve(n);

        // Output the result
        println!("{}", result);
    }
}