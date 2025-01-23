// Question 2: Cherry Pickup II
// How to maximize the cherries collected by two robots in a grid?

fn question_2(grid: Vec<Vec<i32>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dp = vec![vec![vec![-1; cols]; cols]; rows];

    fn dfs(
        row: usize,
        col1: usize,
        col2: usize,
        grid: &Vec<Vec<i32>>,
        dp: &mut Vec<Vec<Vec<i32>>>,
    ) -> i32 {
        if col1 < 0 || col1 >= grid[0].len() || col2 < 0 || col2 >= grid[0].len() {
            return 0;
        }

        if dp[row][col1][col2] != -1 {
            return dp[row][col1][col2];
        }

        let mut result = grid[row][col1];
        if col1 != col2 {
            result += grid[row][col2];
        }

        if row < grid.len() - 1 {
            let mut max_next = 0;
            for new_col1 in col1 as isize - 1..=col1 as isize + 1 {
                for new_col2 in col2 as isize - 1..=col2 as isize + 1 {
                    max_next = max_next.max(dfs(
                        row + 1,
                        new_col1 as usize,
                        new_col2 as usize,
                        grid,
                        dp,
                    ));
                }
            }
            result += max_next;
        }

        dp[row][col1][col2] = result;
        result
    }

    dfs(0, 0, cols - 1, &grid, &mut dp)
}
