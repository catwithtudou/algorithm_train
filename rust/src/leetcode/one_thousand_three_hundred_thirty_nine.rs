pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;

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
      right: None
    }
  }
}

impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs1(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();
                n.val + dfs1(&n.left) + dfs1(&n.right)
            } else {
                0
            }
        }

        fn dfs2(node: &Option<Rc<RefCell<TreeNode>>>, total: i32, ans: &mut i64) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();
                let s = n.val + dfs2(&n.left, total, ans) + dfs2(&n.right, total, ans);
                *ans = (*ans).max(s as i64 * (total - s) as i64);
                s
            } else {
                0
            }
        }

        let total = dfs1(&root);

        let mut ans = 0;
        dfs2(&root, total, &mut ans);

        (ans % 1_000_000_007) as _
    }
}