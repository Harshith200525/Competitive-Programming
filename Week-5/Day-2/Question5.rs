// 73. Set Matrix Zeroes

// Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.

// You must do it in place.

 

// Example 1:


// Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
// Output: [[1,0,1],[0,0,0],[1,0,1]]
// Example 2:


// Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
// Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
 

// Constraints:

// m == matrix.length
// n == matrix[0].length
// 1 <= m, n <= 200
// -231 <= matrix[i][j] <= 231 - 1

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() {
            return;
        }

        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut row_zero = false;
        let mut col_zero = false;

        // Step 2: Determine if the first row or first column should be set to zero
        for r in 0..rows {
            if matrix[r][0] == 0 {
                col_zero = true;
                break;
            }
        }

        for c in 0..cols {
            if matrix[0][c] == 0 {
                row_zero = true;
                break;
            }
        }

        // Step 3: Use the first row and first column to mark rows and columns to be zeroed
        for r in 1..rows {
            for c in 1..cols {
                if matrix[r][c] == 0 {
                    matrix[r][0] = 0;
                    matrix[0][c] = 0;
                }
            }
        }

        // Step 4: Zero out cells based on markers in the first row and column
        for r in 1..rows {
            if matrix[r][0] == 0 {
                for c in 1..cols {
                    matrix[r][c] = 0;
                }
            }
        }

        for c in 1..cols {
            if matrix[0][c] == 0 {
                for r in 1..rows {
                    matrix[r][c] = 0;
                }
            }
        }

        // Step 5: Handle the first row and first column separately, if needed
        if row_zero {
            for c in 0..cols {
                matrix[0][c] = 0;
            }
        }

        if col_zero {
            for r in 0..rows {
                matrix[r][0] = 0;
            }
        }
    }
}
