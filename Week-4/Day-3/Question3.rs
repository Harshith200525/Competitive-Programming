// 199. Binary Tree Right Side View

// Given the root of a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.

 

// Example 1:


// Input: root = [1,2,3,null,5,null,4]
// Output: [1,3,4]
// Example 2:

// Input: root = [1,null,3]
// Output: [1,3]
// Example 3:

// Input: root = []
// Output: []
 

// Constraints:

// The number of nodes in the tree is in the range [0, 100].
// -100 <= Node.val <= 100

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

// Definition for a binary tree node.
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

pub struct Solution;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut max_depth = -1;

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, depth: i32, map: &mut HashMap<i32, i32>, max_depth: &mut i32) {
            if let Some(n) = node {
                let node = n.borrow();
                if depth > *max_depth {
                    map.insert(depth, node.val);
                    *max_depth = depth;
                }
                if let Some(left) = node.left.clone() {
                    dfs(Some(left), depth + 1, map, max_depth);
                }
                if let Some(right) = node.right.clone() {
                    dfs(Some(right), depth + 1, map, max_depth);
                }
            }
        }

        dfs(root, 0, &mut map, &mut max_depth);

        (0..=max_depth)
            .map(|d| map.get(&d).cloned().unwrap_or(0))
            .collect()
    }
}
