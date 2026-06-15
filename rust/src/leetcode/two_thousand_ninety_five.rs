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
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.as_ref()?.next.is_none() {
            return None;
        }

        let mut slow = &head;
        let mut fast = &head.as_ref()?.next.as_ref()?.next;
        while fast.is_some() && fast.as_ref()?.next.is_some() {
            slow = &slow.as_ref()?.next;
            fast = &fast.as_ref()?.next.as_ref()?.next;
        }

        let mut slow = slow as *const Option<Box<ListNode>> as *mut Option<Box<ListNode>>;
        let slow = unsafe { &mut *slow };
        slow.as_mut()?.next = slow.as_mut()?.next.take()?.next; // 删除 slow 的下一个节点

        head
    }
}
