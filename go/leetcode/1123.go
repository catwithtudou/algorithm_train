package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func lcaDeepestLeaves(root *TreeNode) (ans *TreeNode) {
	maxDepth := -1
	var dfs func(node *TreeNode, depth int) int
	dfs = func(node *TreeNode, depth int) int {
		if node == nil {
			maxDepth = max(depth, maxDepth)
			return depth
		}
		leftDepth := dfs(node.Left, depth+1)
		rightDepth := dfs(node.Right, depth+1)
		if leftDepth == rightDepth && leftDepth == maxDepth {
			ans = node
		}
		return max(leftDepth, rightDepth)
	}
	dfs(root, 0)
	return
}
