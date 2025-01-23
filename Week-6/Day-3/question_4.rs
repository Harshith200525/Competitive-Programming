// Question 4: Binary Tree Maximum Path Sum
// How to calculate the maximum path sum in a binary tree using recursion in Rust?

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn question_4(root: Option<Box<TreeNode>>) -> i32 {
    fn helper(node: Option<&Box<TreeNode>>, max_sum: &mut i32) -> i32 {
        if let Some(n) = node {
            let left = helper(n.left.as_deref(), max_sum).max(0);
            let right = helper(n.right.as_deref(), max_sum).max(0);
            *max_sum = (*max_sum).max(left + right + n.val);
            n.val + left.max(right)
        } else {
            0
        }
    }
    let mut max_sum = i32::MIN;
    helper(root.as_deref(), &mut max_sum);
    max_sum
}
