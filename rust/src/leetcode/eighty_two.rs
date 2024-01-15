pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut prev = None;

        while let Some(mut node) = head {
            head = node.next.take();
            let node_val = node.val;
            if head.as_ref().map_or(true, |next| next.val != node.val)
                && prev.map_or(true, |val| val != node_val) {
                tail.next = Some(node);
                tail = tail.next.as_mut().unwrap();
            }
            prev = Some(node_val);
        }
        dummy.next
    }
}