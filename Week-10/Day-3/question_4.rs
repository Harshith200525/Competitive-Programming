// Question 4: Find Median from Data Stream
// How to design a system to compute the median from a stream of numbers?

use std::collections::BinaryHeap;

struct MedianFinder {
    small: BinaryHeap<i32>,
    large: BinaryHeap<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            small: BinaryHeap::new(),
            large: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.small.push(num);
        if let Some(small_top) = self.small.pop() {
            self.large.push(-small_top);
        }

        if self.small.len() < self.large.len() {
            if let Some(large_top) = self.large.pop() {
                self.small.push(-large_top);
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.small.len() > self.large.len() {
            *self.small.peek().unwrap() as f64
        } else {
            (*self.small.peek().unwrap() as f64 - *self.large.peek().unwrap() as f64) / 2.0
        }
    }
}
