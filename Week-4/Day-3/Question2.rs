// 147. Insertion Sort List

// Given the head of a singly linked list, sort the list using insertion sort, and return the sorted list's head.

// The steps of the insertion sort algorithm:

// Insertion sort iterates, consuming one input element each repetition and growing a sorted output list.
// At each iteration, insertion sort removes one element from the input data, finds the location it belongs within the sorted list and inserts it there.
// It repeats until no input elements remain.
// The following is a graphical example of the insertion sort algorithm. The partially sorted list (black) initially contains only the first element in the list. One element (red) is removed from the input data and inserted in-place into the sorted list with each iteration.


 

// Example 1:


// Input: head = [4,2,1,3]
// Output: [1,2,3,4]
// Example 2:


// Input: head = [-1,5,3,4,0]
// Output: [-1,0,3,4,5]
 

// Constraints:

// The number of nodes in the list is in the range [1, 5000].
// -5000 <= Node.val <= 5000

use std::rc::Rc;
use std::cell::RefCell;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub struct Solution;

impl Solution {
    pub fn insertion_sort_list(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
        // Step 1: Extract values from the linked list
        let mut values = vec![];
        let mut current = head.clone();

        while let Some(node) = current {
            let node = node.borrow();
            values.push(node.val);
            current = node.next.clone();
        }

        // Step 2: Sort values in ascending order
        values.sort();

        // Step 3: Rebuild the sorted linked list
        let mut dummy = Rc::new(RefCell::new(ListNode::new(0)));
        let mut tail = dummy.clone();

        for &val in values.iter() {
            let new_node = Rc::new(RefCell::new(ListNode::new(val)));
            tail.borrow_mut().next = Some(new_node.clone());
            tail = new_node;
        }

        dummy.borrow().next.clone()
    }
}
