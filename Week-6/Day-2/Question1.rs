// 378. Kth Smallest Element in a Sorted Matrix

// Given an n x n matrix where each of the rows and columns is sorted in ascending order, return the kth smallest element in the matrix.

// Note that it is the kth smallest element in the sorted order, not the kth distinct element.

// You must find a solution with a memory complexity better than O(n2).

 

// Example 1:

// Input: matrix = [[1,5,9],[10,11,13],[12,13,15]], k = 8
// Output: 13
// Explanation: The elements in the matrix are [1,5,9,10,11,12,13,13,15], and the 8th smallest number is 13
// Example 2:

// Input: matrix = [[-5]], k = 1
// Output: -5
 

// Constraints:

// n == matrix.length == matrix[i].length
// 1 <= n <= 300
// -109 <= matrix[i][j] <= 109
// All the rows and columns of matrix are guaranteed to be sorted in non-decreasing order.
// 1 <= k <= n2


use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Solution;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut min_heap = BinaryHeap::new();
        let row_count = matrix.len();
        
        // Push the first element of each row into the heap
        for i in 0..std::cmp::min(row_count, k as usize) {
            min_heap.push(Reverse((matrix[i][0], i, 0)));
        }

        let mut numbers_checked = 0;
        let mut smallest_element = 0;

        while let Some(Reverse((val, row_index, col_index))) = min_heap.pop() {
            numbers_checked += 1;
            smallest_element = val;

            // If we have checked k elements, return the k-th smallest
            if numbers_checked == k {
                break;
            }

            // If there is a next element in the current row, add it to the heap
            if col_index + 1 < matrix[row_index].len() {
                min_heap.push(Reverse((matrix[row_index][col_index + 1], row_index, col_index + 1)));
            }
        }

        smallest_element
    }
}