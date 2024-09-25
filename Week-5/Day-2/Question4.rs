// 98. Validate Binary Search Tree

// Given the root of a binary tree, determine if it is a valid binary search tree (BST).

// A valid BST is defined as follows:

// The left 
// subtree
//  of a node contains only nodes with keys less than the node's key.
// The right subtree of a node contains only nodes with keys greater than the node's key.
// Both the left and right subtrees must also be binary search trees.
 

// Example 1:


// Input: root = [2,1,3]
// Output: true
// Example 2:


// Input: root = [5,1,4,null,null,3,6]
// Output: false
// Explanation: The root node's value is 5 but its right child's value is 4.
 

// Constraints:

// The number of nodes in the tree is in the range [1, 104].
// -231 <= Node.val <= 231 - 1

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // Helper function for recursion
        fn valid(node: Option<Rc<RefCell<TreeNode>>>, min: i32, max: i32) -> bool {
            if let Some(n) = node {
                let val = n.borrow().val;
                
                // Check if the current node's value is within the valid range
                if val <= min || val >= max {
                    return false;
                }
                
                // Recursively validate left and right subtrees
                valid(n.borrow().left.clone(), min, val) && 
                valid(n.borrow().right.clone(), val, max)
            } else {
                true // An empty node (None) is valid
            }
        }
        
        valid(root, i32::MIN, i32::MAX)
    }
}
