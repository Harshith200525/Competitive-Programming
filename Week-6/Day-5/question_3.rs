// Question 3: House Robber III
// How to find the maximum sum of nodes in a binary tree such that no two adjacent nodes are robbed using Rust?

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn question_3(root: Option<Box<TreeNode>>) -> i32 {
    fn rob_subtree(node: Option<&Box<TreeNode>>) -> (i32, i32) {
        if let Some(n) = node {
            let (left_robbed, left_skipped) = rob_subtree(n.left.as_deref());
            let (right_robbed, right_skipped) = rob_subtree(n.right.as_deref());
            let robbed = n.val + left_skipped + right_skipped;
            let skipped = left_robbed.max(left_skipped) + right_robbed.max(right_skipped);
            (robbed, skipped)
        } else {
            (0, 0)
        }
    }

    let (robbed, skipped) = rob_subtree(root.as_deref());
    robbed.max(skipped)
}
