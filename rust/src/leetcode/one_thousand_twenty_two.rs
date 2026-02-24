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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, val: i32, ans: &mut i32) {
            if let None = node {
                return;
            }
            let node = node.as_ref().unwrap().borrow();
            let val = val << 1 | node.val;
            if node.left.is_none() && node.right.is_none() {
                *ans += val;
                return;
            }
            dfs(&node.left, val, ans);
            dfs(&node.right, val, ans);
        }
        dfs(&root, 0, &mut ans);
        ans
    }
}
