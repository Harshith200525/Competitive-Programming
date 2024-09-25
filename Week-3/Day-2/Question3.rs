// 59. Spiral Matrix II

// Given a positive integer n, generate an n x n matrix filled with elements from 1 to n2 in spiral order.

 

// Example 1:


// Input: n = 3
// Output: [[1,2,3],[8,9,4],[7,6,5]]
// Example 2:

// Input: n = 1
// Output: [[1]]
 

// Constraints:

// 1 <= n <= 20

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut arr = vec![vec![0; n as usize]; n as usize];
        let mut top = 0;
        let mut bottom = n - 1;
        let mut left = 0;
        let mut right = n - 1;
        let mut val = 1;

        while left <= right && top <= bottom {
            for j in left..=right {
                arr[top as usize][j as usize] = val;
                val += 1;
            }
            top += 1;

            for i in top..=bottom {
                arr[i as usize][right as usize] = val;
                val += 1;
            }
            right -= 1;

            if top <= bottom {
                for j in (left..=right).rev() {
                    arr[bottom as usize][j as usize] = val;
                    val += 1;
                }
                bottom -= 1;
            }

            if left <= right {
                for i in (top..=bottom).rev() {
                    arr[i as usize][left as usize] = val;
                    val += 1;
                }
                left += 1;
            }
        }

        return arr;
    }
}