package leetcode

/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Children []*Node
 * }
 */

type NTreeNode struct {
	Val      int
	Children []*NTreeNode
}

func levelNOrder(root *NTreeNode) [][]int {
	var ans [][]int
	if root == nil {
		return ans
	}

	cur := []*NTreeNode{root}
	for len(cur) > 0 {
		var nxt []*NTreeNode
		vals := make([]int, len(cur))
		for i, node := range cur {
			vals[i] = node.Val
			nxt = append(nxt, node.Children...)
		}
		ans = append(ans, vals)
		cur = nxt
	}

	return ans
}

func levelNOrderOther(root *NTreeNode) [][]int {
	var ans [][]int
	if root == nil {
		return ans
	}

	q := []*NTreeNode{root}
	for len(q) > 0 {
		vals := make([]int, len(q))
		for i := range vals {
			node := q[0]
			q = q[1:]
			vals[i] = node.Val
			q = append(q, node.Children...)
		}
		ans = append(ans, vals)
	}
	return ans
}
