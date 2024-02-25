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
	pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
		fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
			if let Some(node) = root {
				let n = node.borrow();
				dfs(n.left.as_ref(), vals);
				vals.push(n.val);
				dfs(n.right.as_ref(), vals);
			}
		}
		let mut vals = Vec::new();
		dfs(root.as_ref(), &mut vals);

		let mut ans = Vec::new();
		let n = vals.len();
		for q in queries {
			let mut j = vals.partition_point(|&x| x < q);
			let mx = if j < n { vals[j] } else { -1 };
			let mn = if j < n && vals[j] == q { q } else if j > 0 { vals[j - 1] } else { -1 };
			ans.push(vec![mn, mx]);
		}

		ans
	}
}