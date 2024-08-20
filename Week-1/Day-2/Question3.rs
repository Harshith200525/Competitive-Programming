// 38. Count and Say
// The count-and-say sequence is a sequence of digit strings defined by the recursive formula:

// countAndSay(1) = "1"
// countAndSay(n) is the run-length encoding of countAndSay(n - 1).
// Run-length encoding (RLE) is a string compression method that works by replacing consecutive identical characters (repeated 2 or more times) with the concatenation of the character and the number marking the count of the characters (length of the run). For example, to compress the string "3322251" we replace "33" with "23", replace "222" with "32", replace "5" with "15" and replace "1" with "11". Thus the compressed string becomes "23321511".

// Given a positive integer n, return the nth element of the count-and-say sequence.

// Input: n = 4

// Output: "1211"

// Explanation:

// countAndSay(1) = "1"
// countAndSay(2) = RLE of "1" = "11"
// countAndSay(3) = RLE of "11" = "21"
// countAndSay(4) = RLE of "21" = "1211"

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let n = n as usize;
        if n<=0 {
            return String::new();
        }

        let mut out = String::from("1");

        for _ in 0..n-1 {
            let mut str_out = String::new();
            let chars: Vec<char> = out.chars().collect();
            let mut prev_char = chars[0];
            let mut count = 1;

            for j in 1..chars.len() {
                if chars[j] == prev_char {
                    count += 1;
                }else{
                    str_out.push_str(&format!("{}{}", count, prev_char));
                    prev_char = chars[j];
                    count = 1;
                }
            }
            str_out.push_str(&format!("{}{}", count, prev_char));
            out = str_out;
        }
        return out
    }

}