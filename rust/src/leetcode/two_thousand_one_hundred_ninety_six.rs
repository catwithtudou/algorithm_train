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
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let n = descriptions.len();
        let mut nodes = HashMap::with_capacity(n + 1);
        let mut child = HashSet::with_capacity(n);

        for d in descriptions {
            let x = d[0];
            let y = d[1];

            nodes
                .entry(x)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(x))));
            nodes
                .entry(y)
                .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(y))));

            if d[2] == 1 {
                nodes.get(&x)?.borrow_mut().left = nodes.get(&y).cloned();
            } else {
                nodes.get(&x)?.borrow_mut().right = nodes.get(&y).cloned();
            }

            child.insert(y);
        }

        for (x, node) in nodes {
            if !child.contains(&x) {
                return Some(node);
            }
        }

        None
    }
}
