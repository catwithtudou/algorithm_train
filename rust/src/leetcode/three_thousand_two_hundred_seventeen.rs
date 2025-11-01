pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

use std::collections::HashSet;

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let set = nums.into_iter().collect::<HashSet<_>>();
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut cur = &mut dummy;
        while let Some(ref mut nxt) = cur.next {
            if set.contains(&nxt.val) {
                cur.next = nxt.next.take();
            } else {
                cur = cur.next.as_mut()?;
            }
        }
        dummy.next
    }
}
