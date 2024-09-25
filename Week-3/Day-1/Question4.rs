// 168. Excel Sheet Column Title

// Given an integer columnNumber, return its corresponding column title as it appears in an Excel sheet.

// For example:

// A -> 1
// B -> 2
// C -> 3
// ...
// Z -> 26
// AA -> 27
// AB -> 28 
// ...
 

// Example 1:

// Input: columnNumber = 1
// Output: "A"
// Example 2:

// Input: columnNumber = 28
// Output: "AB"
// Example 3:

// Input: columnNumber = 701
// Output: "ZY"
 

// Constraints:

// 1 <= columnNumber <= 231 - 1

impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut res = String::new();
        
        while column_number > 0 {
            column_number -= 1;
            let reminder = (column_number % 26) as u8;
            let letter = (b'A' + reminder) as char;
            res.insert(0, letter);
            column_number /= 26;
        }
        
        return res;
    }
}