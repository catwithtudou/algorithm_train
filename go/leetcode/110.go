package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func isBalanced110(root *TreeNode) bool {
	return getHeight(root) != -1
}

func getHeight(node *TreeNode) int {
	if node == nil {
		return 0
	}
	leftH := getHeight(node.Left)
	rightH := getHeight(node.Right)

	if leftH == -1 || rightH == -1 || abs(leftH-rightH) > 1 {
		return -1
	}

	return max(leftH, rightH) + 1
}
