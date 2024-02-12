package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func postorderTraversal(root *TreeNode) []int {
	var postorder func(treeNode *TreeNode)
	res := make([]int, 0)
	postorder = func(treeNode *TreeNode) {
		if treeNode == nil {
			return
		}
		postorder(treeNode.Left)
		postorder(treeNode.Right)
		res = append(res, treeNode.Val)
	}
	postorder(root)
	return res
}
