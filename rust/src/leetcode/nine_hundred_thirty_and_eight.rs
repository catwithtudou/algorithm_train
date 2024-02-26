use std::cell::RefCell;
use std::rc::Rc;

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

pub struct Solution;

impl Solution {
	pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
		let mut sum = 0;
		if let Some(node) = root.as_ref() {
			let x = node.borrow_mut().val;

			if low <= x && x <= high {
				sum = x;
			}

			if x > low {
				sum += Self::range_sum_bst(node.borrow_mut().left.take(), low, high);
			}

			if x < high {
				sum += Self::range_sum_bst(node.borrow_mut().right.take(), low, high);
			}
		}
		sum
	}
}