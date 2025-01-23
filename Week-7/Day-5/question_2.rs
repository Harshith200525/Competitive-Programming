// Question 2: Count of Smaller Numbers After Self
// How to count the number of smaller elements after each element in an array using Rust?

use std::collections::HashMap;

fn question_2(nums: Vec<i32>) -> Vec<i32> {
    let mut bit = BinaryIndexedTree::new(nums.len());
    let mut result = Vec::new();

    for &num in nums.iter().rev() {
        result.push(bit.query(num as usize - 1));
        bit.update(num as usize, 1);
    }

    result.reverse();
    result
}

struct BinaryIndexedTree {
    tree: Vec<i32>,
}

impl BinaryIndexedTree {
    fn new(size: usize) -> Self {
        BinaryIndexedTree {
            tree: vec![0; size + 1],
        }
    }

    fn update(&mut self, index: usize, delta: i32) {
        let mut idx = index as usize + 1;
        while idx < self.tree.len() {
            self.tree[idx] += delta;
            idx += idx & !idx;
        }
    }

    fn query(&self, index: usize) -> i32 {
        let mut sum = 0;
        let mut idx = index as usize + 1;
        while idx > 0 {
            sum += self.tree[idx];
            idx -= idx & !idx;
        }
        sum
    }
}
