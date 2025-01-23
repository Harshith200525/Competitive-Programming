// Question 3: Binary Tree Maximum Path Sum
// How to find the maximum path sum in a binary tree using recursive tree traversal?

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn question_3(root: Option<Box<TreeNode>>) -> i32 {
    fn dfs(node: &Option<Box<TreeNode>>, max_sum: &mut i32) -> i32 {
        if let Some(n) = node {
            let left = dfs(&n.left, max_sum).max(0);
            let right = dfs(&n.right, max_sum).max(0);
            *max_sum = (*max_sum).max(left + right + n.val);
            left.max(right) + n.val
        } else {
            0
        }
    }

    let mut max_sum = i32::MIN;
    dfs(&root, &mut max_sum);
    max_sum
}
