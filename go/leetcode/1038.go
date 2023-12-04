package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func bstToGst(root *TreeNode) *TreeNode {
	lastVal := 0
	var dfs func(root *TreeNode)
	dfs = func(root *TreeNode) {
		if root == nil {
			return
		}
		dfs(root.Right)
		root.Val += lastVal
		lastVal = root.Val
		dfs(root.Left)
	}
	dfs(root)
	return root
}
