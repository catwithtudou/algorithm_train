use std::cell::RefCell;
use std::rc::Rc;

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

pub struct Solution;

impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, mut mask: i32) -> i32 {
            if let Some(x) = node {
                let x = x.borrow();
                mask ^= 1 << x.val;
                if x.left.is_none() && x.right.is_none() {
                    return if (mask & (mask - 1)) == 0 { 1 } else { 0 };
                }
                return dfs(x.left.as_ref(), mask) + dfs(x.right.as_ref(), mask);
            }
            0
        }
        dfs(root.as_ref(), 0)
    }
}

#[cfg(test)]
mod one_thousand_four_hundred_fifty_seven_test {
    use super::*;

    #[test]
    fn test_pseudo_palindromic_paths() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(Solution::pseudo_palindromic_paths(root), 2);
    }
}