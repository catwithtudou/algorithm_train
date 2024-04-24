use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

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


impl Solution {
	pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
		let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
		let mut visited: HashSet<i32> = HashSet::new();

		fn dfs(node_opt: Option<&Rc<RefCell<TreeNode>>>, graph: &mut HashMap<i32, Vec<i32>>) {
			if let Some(node_ref) = node_opt {
				let node = node_ref.borrow();
				if let Some(ref left) = &node.left {
					graph.entry(node.val).or_insert_with(Vec::new).push(left.borrow().val);
					graph.entry(left.borrow().val).or_insert_with(Vec::new).push(node.val);
					dfs(Some(&left), graph);
				}
				if let Some(ref right) = &node.right {
					graph.entry(node.val).or_insert_with(Vec::new).push(right.borrow().val);
					graph.entry(right.borrow().val).or_insert_with(Vec::new).push(node.val);
					dfs(Some(&right), graph);
				}
			}
		}

		if let Some(node) = root.as_ref() {
			dfs(Some(&node), &mut graph);
		}

		let mut q = VecDeque::new();
		let mut time = 0;
		q.push_back((start, 0));
		visited.insert(start);
		while let Some((node_val, curr_time)) = q.pop_front() {
			time = curr_time;
			if let Some(children) = graph.get(&node_val) {
				for &child_value in children {
					if visited.contains(&child_value) {
						continue;
					}
					q.push_back((child_value, time + 1));
					visited.insert(child_value);
				}
			}
		}


		time
	}
}