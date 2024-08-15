use std::cell::RefCell;
use std::collections::HashSet;
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

struct FindElements {
	s: HashSet<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
	fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
		fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, val: i32, s: &mut HashSet<i32>) {
			if let Some(x) = node {
				s.insert(val);
				let x = x.borrow();
				dfs(x.left.as_ref(), val * 2 + 1, s);
				dfs(x.right.as_ref(), val * 2 + 2, s);
			}
		}
		let mut s = HashSet::new();
		dfs(root.as_ref(), 0, &mut s);
		Self { s }
	}

	fn find(&self, target: i32) -> bool {
		self.s.contains(&target)
	}
}

// /**
//  * Your FindElements object will be instantiated and called as such:
//  * let obj = FindElements::new(root);
//  * let ret_1: bool = obj.find(target);
//  */