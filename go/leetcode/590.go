package leetcode

/**
 * Definition for a Node.
 * type Node struct {
 *     Val int
 *     Children []*Node
 * }
 */

func postorder(root *NTreeNode) []int {
	var ans []int
	var dfs func(*NTreeNode)
	dfs = func(node *NTreeNode) {
		if node == nil {
			return
		}
		for _, child := range node.Children {
			dfs(child)
		}
		ans = append(ans, node.Val)
	}
	dfs(root)
	return ans
}

func postorderOther(root *NTreeNode) []int {
	var ans []int
	if root == nil {
		return ans
	}

	st := []*NTreeNode{root}
	vis := map[*NTreeNode]bool{}
	for len(st) > 0 {
		node := st[len(st)-1]
		if len(node.Children) == 0 || vis[node] {
			ans = append(ans, node.Val)
			st = st[:len(st)-1]
			continue
		}

		for i := len(node.Children) - 1; i >= 0; i-- {
			st = append(st, node.Children[i])
		}
		vis[node] = true
	}

	return ans
}
