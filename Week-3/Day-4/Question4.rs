// 105. Construct Binary Tree from Preorder and Inorder Traversal

// Given two integer arrays preorder and inorder where preorder is the preorder traversal of a binary tree and inorder is the inorder traversal of the same tree, construct and return the binary tree.

 

// Example 1:


// Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
// Output: [3,9,20,null,null,15,7]
// Example 2:

// Input: preorder = [-1], inorder = [-1]
// Output: [-1]
 

// Constraints:

// 1 <= preorder.length <= 3000
// inorder.length == preorder.length
// -3000 <= preorder[i], inorder[i] <= 3000
// preorder and inorder consist of unique values.
// Each value of inorder also appears in preorder.
// preorder is guaranteed to be the preorder traversal of the tree.
// inorder is guaranteed to be the inorder traversal of the tree.

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let preorder = VecDeque::from(preorder);
        Self::build_tree_helper(preorder, inorder)
    }
    
    fn build_tree_helper(preorder: VecDeque<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.is_empty() {
            return None;
        }
        
        let root_val = preorder.front().cloned().unwrap();
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        
        let idx = inorder.iter().position(|&x| x == root_val).unwrap();
        
        let (left_inorder, right_inorder) = inorder.split_at(idx);
        let right_inorder = &right_inorder[1..]; // Exclude the root_val itself
        
        let left_preorder = preorder.iter().cloned().filter(|&x| left_inorder.contains(&x)).collect();
        let right_preorder = preorder.iter().cloned().filter(|&x| right_inorder.contains(&x)).collect();
        
        let (left_preorder, right_preorder) = (VecDeque::from(left_preorder), VecDeque::from(right_preorder));
        
        root.borrow_mut().left = Self::build_tree_helper(left_preorder, left_inorder.to_vec());
        root.borrow_mut().right = Self::build_tree_helper(right_preorder, right_inorder.to_vec());
        
        Some(root)
    }
}
