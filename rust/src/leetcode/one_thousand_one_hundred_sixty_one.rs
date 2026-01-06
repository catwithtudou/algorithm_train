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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut row_sum = Vec::new();

        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, level: usize, row_sum: &mut Vec<i32>) {
            if let Some(n) = node {
                let node_borrow = n.borrow();

                if row_sum.len() == level {
                    row_sum.push(node_borrow.val);
                } else {
                    row_sum[level] += node_borrow.val;
                }

                dfs(&node_borrow.left, level + 1, row_sum);
                dfs(&node_borrow.right, level + 1, row_sum);
            }
        }

        dfs(&root, 0, &mut row_sum);

        let mut ans = 0;

        for i in 1..row_sum.len() {
            if row_sum[i] > row_sum[ans] {
                ans = i;
            }
        }

        (ans + 1) as i32
    }
}
