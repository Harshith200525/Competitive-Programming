// Question 1: Falling Squares
// How to simulate the fall of squares on a vertical line and calculate the maximum stack height?

use std::cmp::max;

fn question_1(positions: Vec<Vec<i32>>) -> i32 {
    let mut segment_tree = vec![0; 4 * positions.len()]; // Segment tree
    let mut result = 0;

    // Function to update the segment tree
    fn update(tree: &mut Vec<i32>, index: usize, value: i32, node: usize, start: usize, end: usize) {
        if start == end {
            tree[node] = value;
        } else {
            let mid = (start + end) / 2;
            if index <= mid {
                update(tree, index, value, 2 * node + 1, start, mid);
            } else {
                update(tree, index, value, 2 * node + 2, mid + 1, end);
            }
            tree[node] = max(tree[2 * node + 1], tree[2 * node + 2]);
        }
    }

    // Function to get the maximum height of the stack in the range [l, r]
    fn query(tree: &Vec<i32>, l: usize, r: usize, node: usize, start: usize, end: usize) -> i32 {
        if r < start || l > end {
            return 0;
        }
        if l <= start && end <= r {
            return tree[node];
        }
        let mid = (start + end) / 2;
        let left = query(tree, l, r, 2 * node + 1, start, mid);
        let right = query(tree, l, r, 2 * node + 2, mid + 1, end);
        max(left, right)
    }

    // Process each square drop
    for pos in positions {
        let (left, side_len) = (pos[0] as usize, pos[1]);
        let right = left + side_len - 1;

        // Get the highest position in the range [left, right]
        let max_height = query(&segment_tree, left, right, 0, 0, positions.len() - 1);
        let new_height = max_height + side_len;
        update(&mut segment_tree, left, new_height, 0, 0, positions.len() - 1);

        // Update the result with the maximum height reached
        result = max(result, new_height);
    }

    result
}
