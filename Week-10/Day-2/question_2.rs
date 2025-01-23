// Question 2: Longest Increasing Path in a Matrix
// How to find the longest increasing path in a 2D matrix?

fn question_2(matrix: Vec<Vec<i32>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut memo = vec![vec![0; n]; m];
    let mut max_len = 0;

    fn dfs(matrix: &Vec<Vec<i32>>, i: usize, j: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if memo[i][j] > 0 {
            return memo[i][j];
        }

        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut max_path = 1;

        for (dx, dy) in directions {
            let ni = i as i32 + dx;
            let nj = j as i32 + dy;
            if ni >= 0 && ni < matrix.len() as i32 && nj >= 0 && nj < matrix[0].len() as i32 {
                let ni = ni as usize;
                let nj = nj as usize;
                if matrix[ni][nj] > matrix[i][j] {
                    max_path = max_path.max(1 + dfs(matrix, ni, nj, memo));
                }
            }
        }

        memo[i][j] = max_path;
        max_path
    }

    for i in 0..m {
        for j in 0..n {
            max_len = max_len.max(dfs(&matrix, i, j, &mut memo));
        }
    }

    max_len
}