// Question 4: Escape the Large Maze
// How to determine if it's possible to escape a grid with obstacles using depth-first search?

fn question_4(maze: Vec<Vec<i32>>, start: (i32, i32), end: (i32, i32)) -> bool {
    let mut visited = vec![vec![false; maze[0].len()]; maze.len()];

    fn dfs(maze: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) -> bool {
        if i < 0 || i >= maze.len() || j < 0 || j >= maze[0].len() || maze[i][j] == 1 || visited[i][j] {
            return false;
        }

        visited[i][j] = true;

        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions {
            let new_i = (i as i32 + dx) as usize;
            let new_j = (j as i32 + dy) as usize;

            if new_i == end.0 as usize && new_j == end.1 as usize {
                return true;
            }

            if dfs(maze, visited, new_i, new_j) {
                return true;
            }
        }

        false
    }

    dfs(&maze, &mut visited, start.0 as usize, start.1 as usize)
}
