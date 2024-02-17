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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut cur = head;
        while let Some(mut node) = cur.take() {
            cur = node.next;
            node.next = pre;
            pre = Some(node);
        }
        pre
    }

    pub fn reverse_list_other(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn reverse(mut cur: Option<Box<ListNode>>, mut pre: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if let Some(mut node) = cur.take() {
                cur = node.next;
                node.next = pre;
                pre = Some(node);
                return reverse(cur, pre);
            }
            pre
        }
        reverse(head, None)
    }
}

