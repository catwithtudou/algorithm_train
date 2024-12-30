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

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(
            head: &Option<Box<ListNode>>,
            root: &Option<Rc<RefCell<TreeNode>>>,
            orig_head: &Option<Box<ListNode>>,
        ) -> bool {
            if head.is_none() {
                return true;
            }
            if root.is_none() {
                return false;
            }
            let list_val = head.as_ref().unwrap().val;
            let tree = root.as_ref().unwrap().borrow();
            let tree_val = tree.val;

            list_val == tree_val
                && (dfs(&head.as_ref().unwrap().next, &tree.left, orig_head)
                    || dfs(&head.as_ref().unwrap().next, &tree.right, orig_head))
                || head == orig_head
                    && (dfs(orig_head, &tree.left, orig_head)
                        || dfs(orig_head, &tree.right, orig_head))
        }
        dfs(&head, &root, &head)
    }
}
