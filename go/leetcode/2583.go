package leetcode

import (
	"golang.org/x/exp/slices"
)

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func kthLargestLevelSum(root *TreeNode, k int) int64 {
	var ans []int64
	que := []*TreeNode{root}
	for len(que) > 0 {
		sum := int64(0)
		tmp := que
		que = nil
		for _, node := range tmp {
			sum += int64(node.Val)
			if node.Left != nil {
				que = append(que, node.Left)
			}
			if node.Right != nil {
				que = append(que, node.Right)
			}
		}
		ans = append(ans, sum)
	}

	n := len(ans)
	if k > n {
		return -1
	}
	slices.Sort(ans)
	return ans[n-k]
}
