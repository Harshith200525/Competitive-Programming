// 118. Pascal's Triangle

// Given an integer numRows, return the first numRows of Pascal's triangle.

// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:


 

// Example 1:

// Input: numRows = 5
// Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
// Example 2:

// Input: numRows = 1
// Output: [[1]]
 

// Constraints:

// 1 <= numRows <= 30

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        if num_rows == 0 {
            return result;
        }

        let mut first_row = vec![1];
        result.push(first_row.clone());

        for i in 1..num_rows {
            let prev_row = result[(i - 1) as usize].clone();
            let mut current_row = vec![1];

            for j in 1..i {
                current_row.push(prev_row[(j - 1) as usize] + prev_row[j as usize]);
            }

            current_row.push(1);
            result.push(current_row);
        }

        return result;
    }
}