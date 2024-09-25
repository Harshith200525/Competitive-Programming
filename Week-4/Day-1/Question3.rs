// 206. Reverse Linked List

// Given the head of a singly linked list, reverse the list, and return the reversed list.

 

// Example 1:


// Input: head = [1,2,3,4,5]
// Output: [5,4,3,2,1]
// Example 2:


// Input: head = [1,2]
// Output: [2,1]
// Example 3:

// Input: head = []
// Output: []
 

// Constraints:

// The number of nodes in the list is the range [0, 5000].
// -5000 <= Node.val <= 5000

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Initialize pointers
        let mut prev = None; // Previous node starts as None
        let mut curr = head; // Current node starts at the head

        // Traverse the list
        while let Some(mut node) = curr {
            curr = node.next; // Save the next node
            
            // Reverse the link
            node.next = prev;
            prev = Some(node); // Move prev to the current node
        }

        // prev is now the new head of the reversed list
        prev
    }
}
