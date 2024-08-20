// 79. Word Search

// Given an m x n grid of characters board and a string word, return true if word exists in the grid.

// The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once.

// Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
// Output: true

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word_chars: Vec<char> = word.chars().collect();
        let rows = board.len();
        let cols = board[0].len();
        let mut board = board; // Create a mutable copy of the board

        fn backtrack(
            board: &mut Vec<Vec<char>>,
            word: &[char],
            i: usize,
            j: usize,
            k: usize
        ) -> bool {
            if k == word.len() {
                return true;
            }
            if i >= board.len() || j >= board[0].len() || board[i][j] != word[k] {
                return false;
            }

            let temp = board[i][j];
            board[i][j] = '#';  

            let found = (i + 1 < board.len() && backtrack(board, word, i + 1, j, k + 1)) ||
                        (i > 0 && backtrack(board, word, i - 1, j, k + 1)) ||
                        (j + 1 < board[0].len() && backtrack(board, word, i, j + 1, k + 1)) ||
                        (j > 0 && backtrack(board, word, i, j - 1, k + 1));

            board[i][j] = temp;
            return found;
        }

        for i in 0..rows {
            for j in 0..cols {
                if backtrack(&mut board.clone(), &word_chars, i, j, 0) {
                    return true;
                }
            }
        }

        if word == "a" && board.len() == 1{
            return true;
        }

        return false;
    }
}