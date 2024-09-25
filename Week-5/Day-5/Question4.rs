// 173. Binary Search Tree Iterator

// Implement the BSTIterator class that represents an iterator over the in-order traversal of a binary search tree (BST):

// BSTIterator(TreeNode root) Initializes an object of the BSTIterator class. The root of the BST is given as part of the constructor. The pointer should be initialized to a non-existent number smaller than any element in the BST.
// boolean hasNext() Returns true if there exists a number in the traversal to the right of the pointer, otherwise returns false.
// int next() Moves the pointer to the right, then returns the number at the pointer.
// Notice that by initializing the pointer to a non-existent smallest number, the first call to next() will return the smallest element in the BST.

// You may assume that next() calls will always be valid. That is, there will be at least a next number in the in-order traversal when next() is called.

 

// Example 1:


// Input
// ["BSTIterator", "next", "next", "hasNext", "next", "hasNext", "next", "hasNext", "next", "hasNext"]
// [[[7, 3, 15, null, null, 9, 20]], [], [], [], [], [], [], [], [], []]
// Output
// [null, 3, 7, true, 9, true, 15, true, 20, false]

// Explanation
// BSTIterator bSTIterator = new BSTIterator([7, 3, 15, null, null, 9, 20]);
// bSTIterator.next();    // return 3
// bSTIterator.next();    // return 7
// bSTIterator.hasNext(); // return True
// bSTIterator.next();    // return 9
// bSTIterator.hasNext(); // return True
// bSTIterator.next();    // return 15
// bSTIterator.hasNext(); // return True
// bSTIterator.next();    // return 20
// bSTIterator.hasNext(); // return False
 

// Constraints:

// The number of nodes in the tree is in the range [1, 105].
// 0 <= Node.val <= 106
// At most 105 calls will be made to hasNext, and next.

use std::rc::Rc;
use std::cell::RefCell;
use std::vec::IntoIter;

#[derive(Debug)]
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

    #[inline]
    pub fn with_children(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Self {
        TreeNode {
            val,
            left,
            right,
        }
    }
}

pub struct BSTIterator {
    list: IntoIter<i32>,
}

impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut list = Vec::new();
        Self::in_order(&root, &mut list);
        BSTIterator {
            list: list.into_iter(),
        }
    }

    pub fn next(&mut self) -> i32 {
        self.list.next().unwrap()
    }

    pub fn has_next(&self) -> bool {
        self.list.clone().next().is_some()
    }

    fn in_order(root: &Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            if node.left.is_some() {
                Self::in_order(&node.left, list);
            }
            list.push(node.val);
            if node.right.is_some() {
                Self::in_order(&node.right, list);
            }
        }
    }
}

fn main() {
    // Example usage
    let root = Rc::new(RefCell::new(TreeNode::with_children(
        7,
        Some(Rc::new(RefCell::new(TreeNode::with_children(
            3,
            None,
            None,
        )))),
        Some(Rc::new(RefCell::new(TreeNode::with_children(
            15,
            Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            Some(Rc::new(RefCell::new(TreeNode::new(20)))),
        )))),
    )));

    let mut iterator = BSTIterator::new(Some(root));
    while iterator.has_next() {
        println!("{}", iterator.next());
    }
}
