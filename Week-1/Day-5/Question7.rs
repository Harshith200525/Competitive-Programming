// 72. Edit Distance

// Given two strings word1 and word2, return the minimum number of operations required to convert word1 to word2.

// You have the following three operations permitted on a word:

// Insert a character
// Delete a character
// Replace a character

// Input: word1 = "horse", word2 = "ros"
// Output: 3
// Explanation: 
// horse -> rorse (replace 'h' with 'r')
// rorse -> rose (remove 'r')
// rose -> ros (remove 'e')

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let len1 = word1.len();
        let len2 = word2.len();

        let mut cache = vec![vec![i32::MAX; len2 + 1]; len1 + 1];

        for col in 0..=len2 {
            cache[len1][col] = (len2 - col) as i32;
        }

        for row in 0..=len1 {
            cache[row][len2] = (len1 - row) as i32;
        }

        for i in (0..len1).rev() {
            for j in (0..len2).rev() {
                if word1[i] == word2[j] {
                    cache[i][j] = cache[i + 1][j + 1];
                } else {
                    cache[i][j] = (cache[i + 1][j].min(cache[i][j + 1])).min(cache[i+1][j+1]) + 1;
                }
            }
        }
        return cache[0][0];
    }
}