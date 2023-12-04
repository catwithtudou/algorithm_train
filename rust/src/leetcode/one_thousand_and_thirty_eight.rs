use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

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

impl Solution {
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, s: &mut i32) {
            if let Some(x) = node {
                let mut x = x.borrow_mut();
                dfs(x.right.as_ref(), s);
                *s += x.val;
                x.val = *s;
                dfs(x.left.as_ref(), s);
            }
        }
        let mut s = 0;
        dfs(root.as_ref(), &mut s);
        root
    }
}