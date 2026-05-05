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
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut length = 0;
        let mut cur = &head;

        while let Some(node) = cur {
            length += 1;
            cur = &node.next;
        }

        k %= length;
        if k == 0 {
            return head;
        }

        let mut cur = &mut head;
        for _ in 0..length - k {
            cur = &mut cur.as_mut()?.next;
        }

        let mut new_head = cur.take();

        let mut tail = &mut new_head;

        while !tail.as_mut()?.next.is_none() {
            tail =  &mut tail.as_mut()?.next;
        }

        tail.as_mut()?.next = head;

        new_head
    }
}
