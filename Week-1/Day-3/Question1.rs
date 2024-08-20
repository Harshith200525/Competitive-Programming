// Make The String Great
// Given a string s of lower and upper case English letters.

// A good string is a string which doesn't have two adjacent characters s[i] and s[i + 1] where:

// 0 <= i <= s.length - 2
// s[i] is a lower-case letter and s[i + 1] is the same letter but in upper-case or vice-versa.
// To make the string good, you can choose two adjacent characters that make the string bad and remove them. You can keep doing this until the string becomes good.

// Return the string after making it good. The answer is guaranteed to be unique under the given constraints.

// Notice that an empty string is also good.

// Input: s = "leEeetcode"
// Output: "leetcode"
// Explanation: In the first step, either you choose i = 1 or i = 2, both will result "leEeetcode" to be reduced to "leetcode".

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut end_position = 0;
        let mut char_array: Vec<char> = s.chars().collect();

        for current_position in 0..s.len() {
            if end_position > 0 && (char_array[current_position] as i32 - char_array[end_position - 1] as i32).abs() == 32 {
                end_position -= 1;
            } else {
                char_array[end_position] = char_array[current_position];
                end_position += 1;
            }
        }

        return char_array[..end_position].iter().collect();
    }
}