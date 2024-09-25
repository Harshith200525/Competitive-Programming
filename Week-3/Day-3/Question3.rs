// 1905. Count Sub Islands

// You are given two m x n binary matrices grid1 and grid2 containing only 0's (representing water) and 1's (representing land). An island is a group of 1's connected 4-directionally (horizontal or vertical). Any cells outside of the grid are considered water cells.

// An island in grid2 is considered a sub-island if there is an island in grid1 that contains all the cells that make up this island in grid2.

// Return the number of islands in grid2 that are considered sub-islands.

 

// Example 1:


// Input: grid1 = [[1,1,1,0,0],[0,1,1,1,1],[0,0,0,0,0],[1,0,0,0,0],[1,1,0,1,1]], grid2 = [[1,1,1,0,0],[0,0,1,1,1],[0,1,0,0,0],[1,0,1,1,0],[0,1,0,1,0]]
// Output: 3
// Explanation: In the picture above, the grid on the left is grid1 and the grid on the right is grid2.
// The 1s colored red in grid2 are those considered to be part of a sub-island. There are three sub-islands.
// Example 2:


// Input: grid1 = [[1,0,1,0,1],[1,1,1,1,1],[0,0,0,0,0],[1,1,1,1,1],[1,0,1,0,1]], grid2 = [[0,0,0,0,0],[1,1,1,1,1],[0,1,0,1,0],[0,1,0,1,0],[1,0,0,0,1]]
// Output: 2 
// Explanation: In the picture above, the grid on the left is grid1 and the grid on the right is grid2.
// The 1s colored red in grid2 are those considered to be part of a sub-island. There are two sub-islands.
 

// Constraints:

// m == grid1.length == grid2.length
// n == grid1[i].length == grid2[i].length
// 1 <= m, n <= 500
// grid1[i][j] and grid2[i][j] are either 0 or 1.

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let n = grid1.len();
        let m = grid1[0].len();
        let mut vst = vec![vec![false; m]; n];
        let mut grid_i = grid1;
        let mut grid_ii = grid2;
        let mut ans = 0;

        fn dfs(
            i: usize,
            j: usize,
            grid_i: &Vec<Vec<i32>>,
            grid_ii: &mut Vec<Vec<i32>>,
            vst: &mut Vec<Vec<bool>>,
            flag: &mut bool
        ) {
            if i >= grid_i.len() || j >= grid_i[0].len() || !grid_ii[i][j] || vst[i][j] {
                return;
            }

            vst[i][j] = true;
            if grid_i[i][j] == 0 {
                *flag = false;
            }

            if i > 0 {
                dfs(i - 1, j, grid_i, grid_ii, vst, flag);
            }
            if j > 0 {
                dfs(i, j - 1, grid_i, grid_ii, vst, flag);
            }
            if i + 1 < grid_i.len() {
                dfs(i + 1, j, grid_i, grid_ii, vst, flag);
            }
            if j + 1 < grid_i[0].len() {
                dfs(i, j + 1, grid_i, grid_ii, vst, flag);
            }
        }

        for i in 0..n {
            for j in 0..m {
                if !vst[i][j] && grid_ii[i][j] == 1 {
                    let mut flag = true;
                    dfs(i, j, &grid_i, &mut grid_ii, &mut vst, &mut flag);
                    if flag {
                        ans += 1;
                    }
                }
            }
        }

        return ans;
    }
}
