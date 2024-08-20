// 36. Valid Sudoku
// Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:

// Each row must contain the digits 1-9 without repetition.
// Each column must contain the digits 1-9 without repetition.
// Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.

// Input: board = 
// [["5","3",".",".","7",".",".",".","."]
// ,["6",".",".","1","9","5",".",".","."]
// ,[".","9","8",".",".",".",".","6","."]
// ,["8",".",".",".","6",".",".",".","3"]
// ,["4",".",".","8",".","3",".",".","1"]
// ,["7",".",".",".","2",".",".",".","6"]
// ,[".","6",".",".",".",".","2","8","."]
// ,[".",".",".","4","1","9",".",".","5"]
// ,[".",".",".",".","8",".",".","7","9"]]
// Output: true

use std::collections::HashSet;
use std::collections::HashMap;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut cols: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut boxes: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

        for r in 0..9{
            for c in 0..9{
                let current = board[r][c];
                if current == '.'{
                    continue;
                }

                if rows.entry(r).or_insert_with(HashSet::new).contains(&current)||
                cols.entry(c).or_insert_with(HashSet::new).contains(&current)||
                boxes.entry((r/3, c/3)).or_insert_with(HashSet::new).contains(&current){
                    return false;
                }

                rows.entry(r).or_insert_with(HashSet::new).insert(current);
                cols.entry(c).or_insert_with(HashSet::new).insert(current);
                boxes.entry((r/3, c/3)).or_insert_with(HashSet::new).insert(current);
            }
        }
        return true;
    }
}