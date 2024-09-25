// 130. Surrounded Regions

// You are given an m x n matrix board containing letters 'X' and 'O', capture regions that are surrounded:

// Connect: A cell is connected to adjacent cells horizontally or vertically.
// Region: To form a region connect every 'O' cell.
// Surround: The region is surrounded with 'X' cells if you can connect the region with 'X' cells and none of the region cells are on the edge of the board.
// A surrounded region is captured by replacing all 'O's with 'X's in the input matrix board.

 

// Example 1:

// Input: board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]

// Output: [["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]

// Explanation:


// In the above diagram, the bottom region is not captured because it is on the edge of the board and cannot be surrounded.

// Example 2:

// Input: board = [["X"]]

// Output: [["X"]]

 

// Constraints:

// m == board.length
// n == board[i].length
// 1 <= m, n <= 200
// board[i][j] is 'X' or 'O'.

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let n = board.len();
        if n == 0 {
            return;
        }
        let m = board[0].len();

        let mut visited = vec![vec![false; m]; n];
        let delrow = [-1, 0, 1, 0];
        let delcol = [0, 1, 0, -1];

        // DFS helper function
        fn dfs(
            row: usize,
            col: usize,
            delrow: &[isize; 4],
            delcol: &[isize; 4],
            visited: &mut Vec<Vec<bool>>,
            board: &mut Vec<Vec<char>>,
        ) {
            let n = board.len();
            let m = board[0].len();
            visited[row][col] = true;

            for i in 0..4 {
                let nrow = row as isize + delrow[i];
                let ncol = col as isize + delcol[i];
                if nrow >= 0 && nrow < n as isize && ncol >= 0 && ncol < m as isize
                    && !visited[nrow as usize][ncol as usize]
                    && board[nrow as usize][ncol as usize] == 'O'
                {
                    dfs(nrow as usize, ncol as usize, delrow, delcol, visited, board);
                }
            }
        }

        // Traverse borders
        for i in 0..n {
            if !visited[i][0] && board[i][0] == 'O' {
                dfs(i, 0, &delrow, &delcol, &mut visited, &mut board);
            }
            if !visited[i][m - 1] && board[i][m - 1] == 'O' {
                dfs(i, m - 1, &delrow, &delcol, &mut visited, &mut board);
            }
        }

        for i in 0..m {
            if !visited[0][i] && board[0][i] == 'O' {
                dfs(0, i, &delrow, &delcol, &mut visited, &mut board);
            }
            if !visited[n - 1][i] && board[n - 1][i] == 'O' {
                dfs(n - 1, i, &delrow, &delcol, &mut visited, &mut board);
            }
        }

        // Mark unvisited 'O's as 'X'
        for i in 0..n {
            for j in 0..m {
                if !visited[i][j] && board[i][j] == 'O' {
                    board[i][j] = 'X';
                }
            }
        }
    }
}
