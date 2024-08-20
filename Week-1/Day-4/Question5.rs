// 51. N-Queens

// The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.

// Given an integer n, return all distinct solutions to the n-queens puzzle. You may return the answer in any order.

// Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space, respectively.

// Input: n = 4
// Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
// Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    if n == 1 {
        return vec![vec!["Q".to_string()]];
    }
    if n == 2 || n == 3 {
        return Vec::new();
    }

    let mut results = Vec::new();
    let mut solution = vec![-1; n as usize];
    Self::solve_n_queens_rec(n, &mut solution, 0, &mut results);
    return results
}

fn solve_n_queens_rec(
    n: i32,
    solution: &mut Vec<i32>,
    row: usize,
    results: &mut Vec<Vec<String>>,
) {
    if row == n as usize {
        results.push(Self::construct_solution_string(solution));
        return;
    }

    for col in 0..n {
        if Self::is_valid_move(row, col, solution) {
            solution[row] = col;
            Self::solve_n_queens_rec(n, solution, row + 1, results);
            solution[row] = -1; // Backtrack
        }
    }
}

fn is_valid_move(proposed_row: usize, proposed_col: i32, solution: &Vec<i32>) -> bool {
    for i in 0..proposed_row {
        let old_col = solution[i];
        let diagonal_offset = proposed_row as i32 - i as i32;

        if old_col == proposed_col
            || old_col == proposed_col - diagonal_offset
            || old_col == proposed_col + diagonal_offset
        {
            return false;
        }
    }
    return true
}

fn construct_solution_string(solution: &Vec<i32>) -> Vec<String> {
    let mut board = Vec::new();
    for &col in solution {
        let mut row_str = vec![".".to_string(); solution.len()];
        row_str[col as usize] = "Q".to_string();
        board.push(row_str.concat());
    }
    return board
}

}