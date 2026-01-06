package leetcode

import "math"

func maxLevelSum(root *TreeNode) (ans int) {
	maxSum := math.MinInt
	q := []*TreeNode{root}

	for level := 1; q != nil; level++ {
		tmp := q
		q = nil
		s := 0

		for _, node := range tmp {
			s += node.Val
			if node.Left != nil {
				q = append(q, node.Left)
			}
			if node.Right != nil {
				q = append(q, node.Right)
			}
		}

		if s > maxSum {
			maxSum = s
			ans = level
		}
	}

	return ans
}
