// 114. Flatten Binary Tree to Linked List

// Given the root of a binary tree, flatten the tree into a "linked list":

// The "linked list" should use the same TreeNode class where the right child pointer points to the next node in the list and the left child pointer is always null.
// The "linked list" should be in the same order as a pre-order traversal of the binary tree.
 

// Example 1:


// Input: root = [1,2,5,3,4,null,6]
// Output: [1,null,2,null,3,null,4,null,5,null,6]
// Example 2:

// Input: root = []
// Output: []
// Example 3:

// Input: root = [0]
// Output: [0]
 

// Constraints:

// The number of nodes in the tree is in the range [0, 2000].
// -100 <= Node.val <= 100

use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node
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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        // Edge case: If the root is None, return immediately.
        if root.is_none() {
            return;
        }

        // Get a mutable reference to the current root.
        let mut current = root.clone();
        
        while let Some(cur) = current {
            let mut cur_borrow = cur.borrow_mut();

            // If the current node has a left child.
            if let Some(left) = cur_borrow.left.clone() {
                let mut last = left.clone();
                
                // Find the rightmost node of the left subtree.
                while let Some(right) = last.borrow().right.clone() {
                    last = right;
                }

                // Attach the original right subtree to the rightmost node of the left subtree.
                last.borrow_mut().right = cur_borrow.right.clone();

                // Move the left subtree to the right.
                cur_borrow.right = Some(left);
                cur_borrow.left = None;
            }

            // Move to the right subtree.
            current = cur_borrow.right.clone();
        }
    }
}
