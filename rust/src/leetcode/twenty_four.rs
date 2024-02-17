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

pub struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        dummy_head.next = head;
        let mut cur = dummy_head.as_mut();
        while let Some(mut node) = cur.next.take() {
            if let Some(mut next) = node.next.take() {
                node.next = next.next.take();
                next.next = Some(node);
                cur.next = Some(next);
                cur = cur.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                cur.next = Some(node);
                cur = cur.next.as_mut().unwrap();
            }
        }
        dummy_head.next
    }
}