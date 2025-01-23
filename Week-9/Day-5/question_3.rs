// Question 3: The Skyline Problem
// How to determine the skyline formed by a set of buildings?

use std::collections::BinaryHeap;

fn question_3(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut events = Vec::new();
    for building in &buildings {
        events.push((building[0], -building[2])); // Start of a building
        events.push((building[1], building[2])); // End of a building
    }
    events.sort();

    let mut result = Vec::new();
    let mut max_heap = BinaryHeap::new();
    max_heap.push(0);

    let mut prev_max = 0;

    for (x, height) in events {
        if height < 0 {
            max_heap.push(-height); // Add building height
        } else {
            if let Some(pos) = max_heap.iter().position(|&h| h == height) {
                max_heap.remove(pos); // Remove building height
            }
        }

        let current_max = *max_heap.peek().unwrap_or(&0);
        if current_max != prev_max {
            result.push(vec![x, current_max]);
            prev_max = current_max;
        }
    }

    result
}
