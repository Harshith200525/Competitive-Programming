// 289. Game of Life

// According to Wikipedia's article: "The Game of Life, also known simply as Life, is a cellular automaton devised by the British mathematician John Horton Conway in 1970."

// The board is made up of an m x n grid of cells, where each cell has an initial state: live (represented by a 1) or dead (represented by a 0). Each cell interacts with its eight neighbors (horizontal, vertical, diagonal) using the following four rules (taken from the above Wikipedia article):

// Any live cell with fewer than two live neighbors dies as if caused by under-population.
// Any live cell with two or three live neighbors lives on to the next generation.
// Any live cell with more than three live neighbors dies, as if by over-population.
// Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
// The next state is created by applying the above rules simultaneously to every cell in the current state, where births and deaths occur simultaneously. Given the current state of the m x n grid board, return the next state.

 

// Example 1:


// Input: board = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]]
// Output: [[0,0,0],[1,0,1],[0,1,1],[0,1,0]]
// Example 2:


// Input: board = [[1,1],[1,0]]
// Output: [[1,1],[1,1]]
 

// Constraints:

// m == board.length
// n == board[i].length
// 1 <= m, n <= 25
// board[i][j] is 0 or 1.

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let rows = board.len();
        let cols = board[0].len();
        let directions = [
            (0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)
        ];
        
        let mut to_change = vec![];

        let valid = |r: usize, c: usize| -> bool {
            r < rows && c < cols
        };

        for r in 0..rows {
            for c in 0..cols {
                let mut live_count = 0;

                for (dr, dc) in directions.iter() {
                    let new_row = r as i32 + dr;
                    let new_col = c as i32 + dc;

                    if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
                        if board[new_row as usize][new_col as usize] == 1 {
                            live_count += 1;
                        }
                    }
                }

                if board[r][c] == 1 {
                    if live_count < 2 || live_count > 3 {
                        to_change.push((r, c, 0));
                    }
                } else {
                    if live_count == 3 {
                        to_change.push((r, c, 1));
                    }
                }
            }
        }

        for &(r, c, next_state) in &to_change {
            board[r][c] = next_state;
        }
    }
}
