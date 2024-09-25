// 295. Find Median from Data Stream

// The median is the middle value in an ordered integer list. If the size of the list is even, there is no middle value, and the median is the mean of the two middle values.

// For example, for arr = [2,3,4], the median is 3.
// For example, for arr = [2,3], the median is (2 + 3) / 2 = 2.5.
// Implement the MedianFinder class:

// MedianFinder() initializes the MedianFinder object.
// void addNum(int num) adds the integer num from the data stream to the data structure.
// double findMedian() returns the median of all elements so far. Answers within 10-5 of the actual answer will be accepted.
 

// Example 1:

// Input
// ["MedianFinder", "addNum", "addNum", "findMedian", "addNum", "findMedian"]
// [[], [1], [2], [], [3], []]
// Output
// [null, null, null, 1.5, null, 2.0]

// Explanation
// MedianFinder medianFinder = new MedianFinder();
// medianFinder.addNum(1);    // arr = [1]
// medianFinder.addNum(2);    // arr = [1, 2]
// medianFinder.findMedian(); // return 1.5 (i.e., (1 + 2) / 2)
// medianFinder.addNum(3);    // arr[1, 2, 3]
// medianFinder.findMedian(); // return 2.0
 

// Constraints:

// -105 <= num <= 105
// There will be at least one element in the data structure before calling findMedian.
// At most 5 * 104 calls will be made to addNum and findMedian.
 

use std::collections::BinaryHeap;

pub struct MedianFinder {
    low: BinaryHeap<i32>,   // Max heap (inverted to use min heap properties)
    high: BinaryHeap<i32>,  // Min heap
}

impl MedianFinder {
    pub fn new() -> Self {
        MedianFinder {
            low: BinaryHeap::new(),
            high: BinaryHeap::new(),
        }
    }
    
    pub fn add_num(&mut self, num: i32) {
        if self.low.is_empty() || *self.low.peek().unwrap() >= num {
            self.low.push(num);
        } else {
            self.high.push(num);
        }

        if self.low.len() > self.high.len() + 1 {
            if let Some(val) = self.low.pop() {
                self.high.push(val);
            }
        } else if self.high.len() > self.low.len() {
            if let Some(val) = self.high.pop() {
                self.low.push(val);
            }
        }
    }
    
    pub fn find_median(&self) -> f64 {
        if self.low.len() > self.high.len() {
            return *self.low.peek().unwrap() as f64;
        }
        let low_val = *self.low.peek().unwrap();
        let high_val = *self.high.peek().unwrap();
        (low_val as f64 + high_val as f64) / 2.0
    }
}

fn main() {
    let mut median_finder = MedianFinder::new();
    median_finder.add_num(1);
    println!("{}", median_finder.find_median()); // 1.0
    median_finder.add_num(2);
    println!("{}", median_finder.find_median()); // 1.5
    median_finder.add_num(3);
    println!("{}", median_finder.find_median()); // 2.0
}
