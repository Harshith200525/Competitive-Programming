// Question 3: Swim in Rising Water
// How to find the minimum time required to reach the bottom-right cell of a grid with rising water in each cell using Rust?

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn question_3(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    
    let mut dist = vec![vec![i32::MAX; m]; n];
    dist[0][0] = grid[0][0];

    let mut pq = BinaryHeap::new();
    pq.push(Reverse((grid[0][0], 0, 0))); // (water level, x, y)

    while let Some(Reverse((water, x, y))) = pq.pop() {
        if x == n - 1 && y == m - 1 {
            return water;
        }

        for (dx, dy) in &directions {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            
            if nx >= 0 && nx < n as i32 && ny >= 0 && ny < m as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                let new_water = water.max(grid[nx][ny]);

                if new_water < dist[nx][ny] {
                    dist[nx][ny] = new_water;
                    pq.push(Reverse((new_water, nx, ny)));
                }
            }
        }
    }

    -1 // In case there's no path
}
