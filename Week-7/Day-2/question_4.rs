// Question 4: N-Queens
// How to find all valid solutions for the N-Queens problem using Rust?

fn question_4(n: i32) -> Vec<Vec<String>> {
    fn is_valid(board: &Vec<String>, row: usize, col: usize) -> bool {
        for i in 0..row {
            if board[i].chars().nth(col).unwrap() == 'Q' {
                return false;
            }
            if col >= row - i && board[i].chars().nth(col - (row - i)).unwrap_or('.') == 'Q' {
                return false;
            }
            if col + row - i < board.len()
                && board[i].chars().nth(col + (row - i)).unwrap_or('.') == 'Q'
            {
                return false;
            }
        }
        true
    }

    fn backtrack(
        n: usize,
        board: &mut Vec<String>,
        row: usize,
        solutions: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            solutions.push(board.clone());
            return;
        }
        for col in 0..n {
            if is_valid(board, row, col) {
                board[row].replace_range(col..col + 1, "Q");
                backtrack(n, board, row + 1, solutions);
                board[row].replace_range(col..col + 1, ".");
            }
        }
    }

    let mut solutions = Vec::new();
    let mut board = vec![".".repeat(n as usize); n as usize];
    backtrack(n as usize, &mut board, 0, &mut solutions);
    solutions
}
