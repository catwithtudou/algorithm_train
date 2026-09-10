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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node:&Option<Rc<RefCell<TreeNode>>>,ans:&mut i32)->(i32,i32) {
            let Some(node) = node else {
                return (0,0);
            };

            let node = node.borrow();
            let (l_sum,l_size) = dfs(&node.left,ans);
            let (r_sum,r_size) = dfs(&node.right,ans);
            let sum = l_sum + r_sum + node.val;
            let size = l_size+ r_size +1;
            if node.val == sum/size {
                *ans+=1;
            }
            (sum,size)
        }

        let mut ans = 0;
        dfs(&root,&mut ans);

        ans
    }
}