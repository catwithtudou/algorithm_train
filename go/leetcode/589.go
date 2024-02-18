package leetcode

/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Children []*Node
 * }
 */

func preorder(root *NTreeNode) []int {
	var ans []int
	var dfs func(*NTreeNode)
	dfs = func(node *NTreeNode) {
		if node == nil {
			return
		}
		ans = append(ans, node.Val)
		for _, child := range node.Children {
			dfs(child)
		}
	}
	dfs(root)
	return ans
}

func preorderOther(root *NTreeNode) []int {
	var ans []int
	if root == nil {
		return ans
	}

	st := []*NTreeNode{root}
	for len(st) > 0 {
		node := st[len(st)-1]
		st = st[:len(st)-1]
		ans = append(ans, node.Val)
		for i := len(node.Children) - 1; i >= 0; i-- {
			st = append(st, node.Children[i])
		}
	}

	return ans
}
