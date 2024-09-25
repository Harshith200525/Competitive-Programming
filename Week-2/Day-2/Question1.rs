// 664. Strange Printer

// There is a strange printer with the following two special properties:

// The printer can only print a sequence of the same character each time.
// At each turn, the printer can print new characters starting from and ending at any place and will cover the original existing characters.
// Given a string s, return the minimum number of turns the printer needed to print it.

 

// Example 1:

// Input: s = "aaabbb"
// Output: 2
// Explanation: Print "aaa" first and then print "bbb".
// Example 2:

// Input: s = "aba"
// Output: 2
// Explanation: Print "aaa" first and then print "b" from the second place of the string, which will cover the existing character 'a'.
 

// Constraints:

// 1 <= s.length <= 100
// s consists of lowercase English letters.

impl Solution {
    fn util(i: usize, j: usize, s: &[char], dp: &mut Vec<Vec<i32>>) -> i32 {
        if i > j{
            return 0;
        }

        if dp[i][j] != -1 {
            return dp[i][j];
        }

        let first_letter = s[i];

        let mut answer = 1 + Self::util(i + 1, j, s, dp);

        for k in i+1..=j {
            if s[k] == first_letter {
                let better_answer = Self::util(i, k - 1, s, dp) + Self::util(k+1, j, s, dp);
                answer = answer.min(better_answer);
            }
        }

        dp[i][j] = answer;
        return answer;

    }
    pub fn strange_printer(s: String) -> i32 {
        let n = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut dp = vec![vec![-1; n]; n];
        return Self::util(0, n - 1, &s, &mut dp);
    }
}