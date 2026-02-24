package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

func sumRootToLeaf(root *TreeNode) (ans int) {
	var dfs func(*TreeNode, int)
	dfs = func(node *TreeNode, val int) {
		if node == nil {
			return
		}

		val = val<<1 | node.Val
		if node.Left == nil && node.Right == nil {
			ans += val
			return
		}
		dfs(node.Left, val)
		dfs(node.Right, val)
	}
	dfs(root, 0)
	return
}
