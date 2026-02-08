pub struct Solution;

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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn get_height(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(node) = node {
                let node = node.borrow();
                let left_height = get_height(&node.left);
                if left_height == -1 {
                    return -1;
                }
                let right_height = get_height(&node.right);
                if right_height == -1 || (left_height - right_height).abs() > 1 {
                    return -1;
                }
                return 1 + left_height.max(right_height);
            }

            0
        }
        get_height(&root) != -1
    }
}
