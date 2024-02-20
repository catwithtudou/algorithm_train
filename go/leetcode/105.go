package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

func buildTree(preorder []int, inorder []int) *TreeNode {
	dicMap := make(map[int]int, len(inorder))
	for i, x := range inorder {
		dicMap[x] = i
	}
	var recur func(int, int, int) *TreeNode
	recur = func(root, left, right int) *TreeNode {
		if left > right {
			return nil
		}
		node := &TreeNode{Val: preorder[root]}
		i := dicMap[preorder[root]]
		node.Left = recur(root+1, left, i-1)
		node.Right = recur(root+1+i-left, i+1, right)
		return node
	}

	return recur(0, 0, len(inorder)-1)
}
