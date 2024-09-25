// 124. Binary Tree Maximum Path Sum

// A path in a binary tree is a sequence of nodes where each pair of adjacent nodes in the sequence has an edge connecting them. A node can only appear in the sequence at most once. Note that the path does not need to pass through the root.

// The path sum of a path is the sum of the node's values in the path.

// Given the root of a binary tree, return the maximum path sum of any non-empty path.

 

// Example 1:


// Input: root = [1,2,3]
// Output: 6
// Explanation: The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 = 6.
// Example 2:


// Input: root = [-10,9,20,null,null,15,7]
// Output: 42
// Explanation: The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 + 7 = 42.
 

// Constraints:

// The number of nodes in the tree is in the range [1, 3 * 104].
// -1000 <= Node.val <= 1000

use std::rc::Rc;
use std::cell::RefCell;

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

pub struct Solution;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_path = std::i32::MIN; // placeholder to be updated

        // Helper function to calculate maximum gain from the current node
        fn get_max_gain(node: Option<Rc<RefCell<TreeNode>>>, max_path: &mut i32) -> i32 {
            if let Some(n) = node {
                let node = n.borrow();
                let left_gain = get_max_gain(node.left.clone(), max_path).max(0); // Read important observations
                let right_gain = get_max_gain(node.right.clone(), max_path).max(0); // Read important observations

                let current_max_path = node.val + left_gain + right_gain; // Read first three images of recursion stack
                *max_path = (*max_path).max(current_max_path); // Update the global max_path

                node.val + left_gain.max(right_gain) // Read the last image of recursion stack
            } else {
                0
            }
        }

        get_max_gain(root, &mut max_path); // Starts the recursion chain
        max_path
    }
}
