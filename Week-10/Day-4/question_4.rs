// Question 4: Serialize and Deserialize Binary Tree
// How to serialize and deserialize a binary tree?

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn serialize(root: Option<Box<TreeNode>>) -> String {
    fn helper(node: &Option<Box<TreeNode>>, result: &mut Vec<String>) {
        if let Some(n) = node {
            result.push(n.val.to_string());
            helper(&n.left, result);
            helper(&n.right, result);
        } else {
            result.push("null".to_string());
        }
    }

    let mut result = Vec::new();
    helper(&root, &mut result);
    result.join(",")
}

fn deserialize(data: String) -> Option<Box<TreeNode>> {
    let mut values = data.split(',').map(|s| s.to_string()).collect::<Vec<String>>();

    fn helper(values: &mut Vec<String>) -> Option<Box<TreeNode>> {
        if values.is_empty() {
            return None;
        }

        let val = values.remove(0);
        if val == "null" {
            return None;
        }

        Some(Box::new(TreeNode {
            val: val.parse().unwrap(),
            left: helper(values),
            right: helper(values),
        }))
    }

    helper(&mut values)
}
