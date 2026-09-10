package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func averageOfSubtree(root *TreeNode) (ans int) {

	var dfs func(*TreeNode) (int, int)
	dfs = func(node *TreeNode) (int, int) {
		if node == nil {
			return 0, 0
		}

		lNum, lSize := dfs(node.Left)
		rNum, rSize := dfs(node.Right)
		sum := lNum + rNum + node.Val
		size := lSize + rSize + 1
		if node.Val == sum/size {
			ans++
		}
		return sum, size
	}

	dfs(root)

	return
}
