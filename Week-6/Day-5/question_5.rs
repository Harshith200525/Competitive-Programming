// Question 5: Top K Frequent Elements
// How to find the K most frequent elements in an array using Rust?

use std::collections::{BinaryHeap, HashMap};

fn question_5(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut frequency_map = HashMap::new();
    for num in nums {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    let mut heap = BinaryHeap::new();
    for (num, freq) in frequency_map {
        heap.push((freq, num));
    }

    heap.into_iter()
        .take(k)
        .map(|(_, num)| num)
        .collect()
}
