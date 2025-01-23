// Question 2: Recover Binary Search Tree
// How to recover a Binary Search Tree where two nodes were swapped during traversal using Rust?

#[derive(Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn question_2(root: &mut Option<Box<TreeNode>>) {
    let mut nodes = vec![];
    let mut prev = None;
    
    inorder_traversal(root, &mut prev, &mut nodes);
    
    if nodes.len() == 2 {
        let (a, b) = &nodes[0];
        let (x, y) = &nodes[1];
        swap(a, b);
        swap(x, y);
    }
}

fn inorder_traversal(root: &mut Option<Box<TreeNode>>, prev: &mut Option<i32>, nodes: &mut Vec<(i32, i32)>) {
    if let Some(node) = root {
        inorder_traversal(&mut node.left, prev, nodes);
        if let Some(prev_val) = *prev {
            if node.val < prev_val {
                nodes.push((node.val, prev_val));
            }
        }
        *prev = Some(node.val);
        inorder_traversal(&mut node.right, prev, nodes);
    }
}

fn swap(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}
