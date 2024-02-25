package leetcode

import "golang.org/x/exp/slices"

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func closestNodes(root *TreeNode, queries []int) [][]int {
	var vals []int

	var dfs func(root *TreeNode)
	dfs = func(root *TreeNode) {
		if root == nil {
			return
		}
		dfs(root.Left)
		vals = append(vals, root.Val)
		dfs(root.Right)
	}
	dfs(root)

	ans := make([][]int, len(queries))
	nodeLen := len(vals)
	for i, v := range queries {
		mn, mx := -1, -1
		j, ok := slices.BinarySearch(vals, v)
		if j < nodeLen {
			mx = vals[j]
		}
		if !ok {
			j--
		}
		if j >= 0 {
			mn = vals[j]
		}

		ans[i] = []int{mn, mx}
	}
	return ans
}
