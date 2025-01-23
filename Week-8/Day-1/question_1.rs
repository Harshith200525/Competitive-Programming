// Question 1: Kth Largest Element in a Stream
// How to find the Kth largest element in a stream of numbers using Rust?

use std::collections::BinaryHeap;

struct KthLargest {
    min_heap: BinaryHeap<i32>,
    k: usize,
}

impl KthLargest {
    fn new(k: usize, nums: Vec<i32>) -> Self {
        let mut min_heap = BinaryHeap::with_capacity(k);
        for num in nums {
            min_heap.push(num);
            if min_heap.len() > k {
                min_heap.pop();
            }
        }
        KthLargest { min_heap, k }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.min_heap.push(val);
        if self.min_heap.len() > self.k {
            self.min_heap.pop();
        }
        *self.min_heap.peek().unwrap()
    }
}
