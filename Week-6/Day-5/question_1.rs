// Question 1: Kth Largest Element in Array
// How to find the Kth largest element in an unsorted array using Rust?

use std::collections::BinaryHeap;

fn question_1(nums: Vec<i32>, k: usize) -> i32 {
    let mut heap = BinaryHeap::new();
    for &num in &nums {
        heap.push(num);
    }
    for _ in 0..k - 1 {
        heap.pop();
    }
    heap.pop().unwrap()
}
