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
    fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if let Some(node) = node {
                let n = node.borrow();
                dfs(&n.left, ans);
                ans.push(n.val);
                dfs(&n.right, ans);
            }
        }

        let mut ans = vec![];
        dfs(&root, &mut ans);
        ans
    }

    fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.is_empty() {
                return None;
            }

            let m = nums.len() / 2;

            Some(Rc::new(RefCell::new(TreeNode {
                val: nums[m],
                left: dfs(&nums[..m]),
                right: dfs(&nums[m + 1..]),
            })))
        }

        dfs(&nums)
    }

    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let nums = Self::inorder_traversal(root);
        Self::sorted_array_to_bst(nums)
    }
}
