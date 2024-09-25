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

#[derive(Debug, PartialEq, Eq, Clone)]
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
        fn flatten_tree(node: &mut Option<Rc<RefCell<TreeNode>>>) {
            if let Some(n) = node {
                let mut node = n.borrow_mut();
                let mut current = Some(node.clone());
                
                // Process left subtree
                if let Some(left) = node.left.take() {
                    flatten_tree(&mut Some(left));
                    
                    let mut right_most = left.clone();
                    while let Some(next) = right_most.borrow().right.clone() {
                        right_most = next;
                    }
                    
                    right_most.borrow_mut().right = node.right.take();
                    node.right = Some(left);
                }
                
                // Move to the right subtree
                flatten_tree(&mut node.right);
            }
        }
        
        flatten_tree(root);
    }
}
