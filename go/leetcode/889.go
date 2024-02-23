package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func constructFromPrePost(preorder, postorder []int) *TreeNode {
	n := len(preorder)
	index := make(map[int]int, n)
	for i, v := range postorder {
		index[v] = i
	}

	var dfs func(int, int, int, int) *TreeNode
	dfs = func(preL int, preR int, posL int, posR int) *TreeNode {
		if preL == preR {
			return nil
		}

		if preL+1 == preR {
			return &TreeNode{Val: preorder[preL]}
		}

		leftSize := index[preorder[preL+1]] - posL + 1
		left := dfs(preL+1, preL+1+leftSize, posL, posL+leftSize)
		right := dfs(preL+1+leftSize, preR, posL+leftSize, posR-1)
		return &TreeNode{Val: preorder[preL], Left: left, Right: right}
	}

	return dfs(0, n, 0, n)
}
