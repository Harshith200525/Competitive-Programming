// 108. Convert Sorted Array to Binary Search Tree

// Given an integer array nums where the elements are sorted in ascending order, convert it to a 
// height-balanced
//  binary search tree.

 

// Example 1:


// Input: nums = [-10,-3,0,5,9]
// Output: [0,-3,9,-10,null,5]
// Explanation: [0,-10,5,null,-3,null,9] is also accepted:

// Example 2:


// Input: nums = [1,3]
// Output: [3,1]
// Explanation: [1,null,3] and [3,1] are both height-balanced BSTs.
 

// Constraints:

// 1 <= nums.length <= 104
// -104 <= nums[i] <= 104
// nums is sorted in a strictly increasing order.

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(arr: &[i32], s: usize, e: usize) -> Option<Rc<RefCell<TreeNode>>> {
            if s > e {
                return None;
            }
            let mid = s + (e - s) / 2;
            let node = Rc::new(RefCell::new(TreeNode::new(arr[mid])));
            let left_child = helper(arr, s, mid - 1);
            let right_child = helper(arr, mid + 1, e);
            if let Some(left) = left_child {
                node.borrow_mut().left = Some(left);
            }
            if let Some(right) = right_child {
                node.borrow_mut().right = Some(right);
            }
            Some(node)
        }
        
        let n = nums.len();
        if n == 0 {
            return None;
        }
        helper(&nums, 0, n - 1)
    }
}
