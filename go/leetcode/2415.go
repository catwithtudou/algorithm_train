package leetcode

// BFS
func reverseOddLevels(root *TreeNode) *TreeNode {
	q := make([]*TreeNode, 0)
	q = append(q, root)
	isOdd := false
	for len(q) > 0 {
		if isOdd {
			n := len(q)
			for i := 0; i < n/2; i++ {
				nodeL, nodeR := q[i], q[n-i-1]
				nodeL.Val, nodeR.Val = nodeR.Val, nodeL.Val
			}
		}

		tmp := make([]*TreeNode, 0, len(q)*2)
		for _, node := range q {
			if node.Left != nil {
				tmp = append(tmp, node.Left, node.Right)
			}
		}
		q = tmp
		isOdd = !isOdd
	}

	return root
}
