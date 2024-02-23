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
	pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
		let mut ans = vec![];
		let mut que = vec![root.unwrap()];
		while !que.is_empty() {
			let mut sum = 0i64;
			let mut tmp = vec![];
			for node in que {
				let mut cur = node.borrow_mut();
				sum += cur.val as i64;
				if let Some(l) = cur.left.take() {
					tmp.push(l);
				}
				if let Some(r) = cur.right.take() {
					tmp.push(r);
				}
			}
			que = tmp;
			ans.push(sum);
		}
		if k as usize > ans.len() {
			return -1;
		}
		ans.sort_unstable();
		ans[ans.len() - k as usize]
	}
}