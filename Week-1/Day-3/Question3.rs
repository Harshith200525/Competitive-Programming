// 6. Zigzag Conversion
// The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)

// P   A   H   N
// A P L S I I G
// Y   I   R
// And then read line by line: "PAHNAPLSIIGYIR"

// Write the code that will take a string and make this conversion given a number of rows:

// string convert(string s, int numRows);

// Input: s = "PAYPALISHIRING", numRows = 3
// Output: "PAHNAPLSIIGYIR"

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
       if num_rows == 1 || num_rows as usize >= s.len() {
        return s;
       } 

       let mut rows: Vec<String> = vec![String::new(); num_rows as usize];
       let mut idx: i32 = 0;
       let mut direction: i32 = 1;

       for char in s.chars() {
        rows[idx as usize].push(char);

        if idx == 0{
            direction = 1;
        }else if idx == num_rows - 1{
            direction = -1;
        }

        idx += direction;
       }

       return rows.concat();
    }
}