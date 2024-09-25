// 763. Partition Labels

// You are given a string s. We want to partition the string into as many parts as possible so that each letter appears in at most one part.

// Note that the partition is done so that after concatenating all the parts in order, the resultant string should be s.

// Return a list of integers representing the size of these parts.

 

// Example 1:

// Input: s = "ababcbacadefegdehijhklij"
// Output: [9,7,8]
// Explanation:
// The partition is "ababcbaca", "defegde", "hijhklij".
// This is a partition so that each letter appears in at most one part.
// A partition like "ababcbacadefegde", "hijhklij" is incorrect, because it splits s into less parts.
// Example 2:

// Input: s = "eccbbbbdec"
// Output: [10]
 

// Constraints:

// 1 <= s.length <= 500
// s consists of lowercase English letters.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last_occurrence = HashMap::new();
        
        // Record the last occurrence of each character
        for (i, ch) in s.chars().enumerate() {
            last_occurrence.insert(ch, i);
        }
        
        let mut result = Vec::new();
        let mut start = 0;
        let mut end = 0;
        
        for (i, ch) in s.chars().enumerate() {
            // Update the end position to be the furthest position of the current character
            end = end.max(*last_occurrence.get(&ch).unwrap());
            
            // If the current position reaches the end position, we can finalize the partition
            if i == end {
                result.push((end - start + 1) as i32);
                start = i + 1;
            }
        }
        
        result
    }
}
