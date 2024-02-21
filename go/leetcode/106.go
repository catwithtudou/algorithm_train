package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func buildTree106(inorder []int, postorder []int) *TreeNode {
	n := len(inorder)
	if n == 0 {
		return nil
	}
	index := make(map[int]int, n)
	for i, v := range inorder {
		index[v] = i
	}

	var dfs func(inL, inR, posL, posR int) *TreeNode
	dfs = func(inL, inR, posL, posR int) *TreeNode {
		if posL == posR {
			return nil
		}
		leftSize := index[postorder[posR-1]] - inL
		left := dfs(inL, inL+leftSize, posL, posL+leftSize)
		right := dfs(inL+leftSize+1, inR, posL+leftSize, posR-1)
		return &TreeNode{postorder[posR-1], left, right}
	}

	return dfs(0, n, 0, n)
}
