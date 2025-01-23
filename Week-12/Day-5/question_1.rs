// Question 1: Trapping Rain Water II
// How to calculate the total water trapped in a 3D elevation map using priority queues and BFS?

use std::collections::BinaryHeap;

fn question_1(height_map: Vec<Vec<i32>>) -> i32 {
    let rows = height_map.len();
    let cols = height_map[0].len();
    if rows < 3 || cols < 3 {
        return 0;
    }

    let mut visited = vec![vec![false; cols]; rows];
    let mut heap = BinaryHeap::new();

    for i in 0..rows {
        for j in [0, cols - 1] {
            heap.push((std::cmp::Reverse(height_map[i][j]), i, j));
            visited[i][j] = true;
        }
    }

    for j in 0..cols {
        for i in [0, rows - 1] {
            heap.push((std::cmp::Reverse(height_map[i][j]), i, j));
            visited[i][j] = true;
        }
    }

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut water = 0;

    while let Some((std::cmp::Reverse(height), x, y)) = heap.pop() {
        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < rows as isize && ny >= 0 && ny < cols as isize {
                let nx = nx as usize;
                let ny = ny as usize;
                if !visited[nx][ny] {
                    visited[nx][ny] = true;
                    water += 0.max(height - height_map[nx][ny]);
                    heap.push((std::cmp::Reverse(height.max(height_map[nx][ny])), nx, ny));
                }
            }
        }
    }

    water
}
