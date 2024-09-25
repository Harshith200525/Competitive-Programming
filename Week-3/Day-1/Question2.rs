// 885. Spiral Matrix III

// You start at the cell (rStart, cStart) of an rows x cols grid facing east. The northwest corner is at the first row and column in the grid, and the southeast corner is at the last row and column.

// You will walk in a clockwise spiral shape to visit every position in this grid. Whenever you move outside the grid's boundary, we continue our walk outside the grid (but may return to the grid boundary later.). Eventually, we reach all rows * cols spaces of the grid.

// Return an array of coordinates representing the positions of the grid in the order you visited them.

 

// Example 1:


// Input: rows = 1, cols = 4, rStart = 0, cStart = 0
// Output: [[0,0],[0,1],[0,2],[0,3]]
// Example 2:


// Input: rows = 5, cols = 6, rStart = 1, cStart = 4
// Output: [[1,4],[1,5],[2,5],[2,4],[2,3],[1,3],[0,3],[0,4],[0,5],[3,5],[3,4],[3,3],[3,2],[2,2],[1,2],[0,2],[4,5],[4,4],[4,3],[4,2],[4,1],[3,1],[2,1],[1,1],[0,1],[4,0],[3,0],[2,0],[1,0],[0,0]]
 

// Constraints:

// 1 <= rows, cols <= 100
// 0 <= rStart < rows
// 0 <= cStart < cols

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut num_steps = 1;
        let total_cells = (rows * cols) as usize;
        let mut result = Vec::with_capacity(total_cells);
        let (mut r, mut c) = (r_start, c_start);
        let mut d = 0;

        while result.len() < total_cells {
            for _ in 0..2 {
                for _ in 0..num_steps {
                    if r >= 0 && r < rows && c >= 0 && c < cols {
                        result.push(vec![r, c]);
                    }
                    if result.len() == total_cells {
                        return result;
                    }
                    r += directions[d].0;
                    c += directions[d].1;
                }
                d = (d + 1) % 4;
            }
            num_steps += 1;
        }

        return result;
    }
}