// Question 1: Unique Binary Search Trees II
// How to generate all structurally unique BSTs for numbers from 1 to n?

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn question_1(n: i32) -> Vec<Option<Box<TreeNode>>> {
    if n == 0 {
        return vec![];
    }

    fn generate_trees(start: i32, end: i32) -> Vec<Option<Box<TreeNode>>> {
        if start > end {
            return vec![None];
        }

        let mut result = Vec::new();
        for i in start..=end {
            let left_trees = generate_trees(start, i - 1);
            let right_trees = generate_trees(i + 1, end);

            for left in left_trees.iter() {
                for right in right_trees.iter() {
                    let mut root = TreeNode {
                        val: i,
                        left: left.clone(),
                        right: right.clone(),
                    };
                    result.push(Some(Box::new(root)));
                }
            }
        }

        result
    }

    generate_trees(1, n)
}
