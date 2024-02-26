package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func rangeSumBST(root *TreeNode, low int, high int) (sum int) {
	if root == nil {
		return
	}

	x := root.Val
	if x >= low && x <= high {
		sum += x
	}

	if x > low {
		sum += rangeSumBST(root.Left, low, high)
	}
	if x < high {
		sum += rangeSumBST(root.Right, low, high)
	}
	return
}
