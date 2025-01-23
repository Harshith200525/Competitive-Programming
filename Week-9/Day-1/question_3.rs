// Question 3: Minimize Deviation in Array
// How to minimize the maximum deviation in the array after applying operations?

use std::collections::BinaryHeap;

fn question_3(nums: Vec<i32>) -> i32 {
    let mut max_heap = BinaryHeap::new();
    let mut min_val = i32::MAX;

    for &num in &nums {
        if num % 2 == 1 {
            max_heap.push(-num * 2); // Double odd values
            min_val = min_val.min(num * 2); // Track min value
        } else {
            max_heap.push(-num); // Push even values as-is
            min_val = min_val.min(num);
        }
    }

    let mut result = i32::MAX;

    while let Some(max_val) = max_heap.pop() {
        result = result.min(-max_val - min_val); // Calculate deviation

        let next_val = -max_val / 2;
        if next_val < min_val {
            break; // No further valid operations possible
        }
        max_heap.push(-next_val);
        min_val = min_val.min(next_val); // Update minimum value
    }

    result
}
