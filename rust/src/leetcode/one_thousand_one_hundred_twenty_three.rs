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
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
            if let Some(node) = node {
                let x = node.borrow();
                let (left_height, left_lca) = dfs(&x.left);
                let (right_height, right_lca) = dfs(&x.right);
                if left_height > right_height {
                    return (left_height + 1, left_lca);
                } else if left_height < right_height {
                    return (right_height + 1, right_lca);
                }
                (left_height + 1, Some(node.clone()))
            } else {
                (0, None)
            }
        }
        dfs(&root).1
    }
}
