use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    pub fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let tree_node = root.clone().unwrap();
        Self::dfs(&tree_node.borrow().left, &tree_node.borrow().right, true);
        root
    }

    fn dfs(node1: &Option<Rc<RefCell<TreeNode>>>, node2: &Option<Rc<RefCell<TreeNode>>>, is_odd: bool) {
        if let (Some(node1), Some(node2)) = (node1, node2) {
            let (mut node1, mut node2) = (node1.borrow_mut(), node2.borrow_mut());
            if is_odd {
                std::mem::swap(&mut node1.val, &mut node2.val);
            }
            Self::dfs(&node1.left, &node2.right, !is_odd);
            Self::dfs(&node1.right, &node2.left, !is_odd);
        }
    }
}