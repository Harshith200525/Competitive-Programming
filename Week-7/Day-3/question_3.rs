// Question 3: Remove Nth Node From End of List
// How to remove the Nth node from the end of a linked list using Rust?

#[derive(PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn question_3(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut fast = &head;
    let mut slow = &mut head;

    // Move fast pointer n steps ahead
    for _ in 0..n {
        if let Some(ref next) = fast {
            fast = &next.next;
        }
    }

    // Move both pointers together until fast reaches the end
    while let Some(ref next) = fast {
        fast = &next.next;
        slow = &mut slow.as_mut().unwrap().next;
    }

    // Remove the Nth node from the end
    if let Some(ref mut node) = slow {
        node.next = node.next.take().and_then(|next| next.next);
    }

    head
}
