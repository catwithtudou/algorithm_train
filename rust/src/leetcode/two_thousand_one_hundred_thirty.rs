pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
        let n = Self::list_len(&head);

        // 找到后半段的起点，并把链表从中间断开
        let mut cur = &mut head;

        for _ in 0..n / 2 {
            cur = &mut cur.as_mut().unwrap().next;
        }

        let second_half = cur.take();
        let reversed_second_half = Self::reverse_list(second_half);

        let mut p1 = head.as_ref();
        let mut p2 = reversed_second_half.as_ref();

        let mut ans = 0;

        while let Some(node2) = p2 {
            let node1 = p1.unwrap();

            ans = ans.max(node1.val + node2.val);

            p1 = node1.next.as_ref();
            p2 = node2.next.as_ref();
        }

        ans
    }

    fn list_len(head: &Option<Box<ListNode>>) -> usize {
        let mut n = 0;
        let mut cur = head.as_ref();

        while let Some(node) = cur {
            n += 1;
            cur = node.next.as_ref();
        }

        n
    }

    fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;

        while let Some(mut cur) = head {
            let nxt = cur.next.take();

            cur.next = pre;
            pre = Some(cur);
            head = nxt;
        }

        pre
    }
}