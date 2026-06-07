package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func createBinaryTree(descriptions [][]int) *TreeNode {
	n := len(descriptions)
	nodes := make(map[int]*TreeNode, n+1)
	child := make(map[int]bool, n)

	for _, d := range descriptions {
		x, y := d[0], d[1]
		if nodes[x] == nil {
			nodes[x] = &TreeNode{Val: x}
		}
		if nodes[y] == nil {
			nodes[y] = &TreeNode{Val: y}
		}
		child[y] = true
		if d[2] == 1 {
			nodes[x].Left = nodes[y]
		} else {
			nodes[x].Right = nodes[y]
		}
	}

	for x := range nodes {
		if !child[x] {
			return nodes[x]
		}
	}

	return nil
}
