// Question 3: Swim in Rising Water
// How to find the minimum time to reach the bottom-right of a grid as water levels rise using BFS and priority queues?

use std::collections::BinaryHeap;

fn question_3(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let mut heap = BinaryHeap::new();
    let mut visited = vec![vec![false; m]; n];

    heap.push((std::cmp::Reverse(grid[0][0]), 0, 0)); // (time, row, col)
    visited[0][0] = true;

    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((std::cmp::Reverse(time), x, y)) = heap.pop() {
        if x == n - 1 && y == m - 1 {
            return time;
        }

        for (dx, dy) in &directions {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;

            if nx < n && ny < m && !visited[nx][ny] {
                visited[nx][ny] = true;
                heap.push((std::cmp::Reverse(time.max(grid[nx][ny])), nx, ny));
            }
        }
    }

    -1
}
