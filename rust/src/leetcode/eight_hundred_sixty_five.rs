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
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut max_depth = -1;

        let mut ans = None;

        fn dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            depth: i32,
            max_depth: &mut i32,
            ans: &mut Option<Rc<RefCell<TreeNode>>>,
        ) -> i32 {
            if let Some(n) = node {
                let left_depth = dfs(&n.borrow().left, depth + 1, max_depth, ans);
                let right_depth = dfs(&n.borrow().right, depth + 1, max_depth, ans);

                if left_depth == right_depth && left_depth >= *max_depth {
                    *ans = Some(n.clone());
                }

                return left_depth.max(right_depth);
            }

            *max_depth = depth.max(*max_depth);

            depth
        }

        dfs(&root, 0, &mut max_depth, &mut ans);
        ans
    }
}
