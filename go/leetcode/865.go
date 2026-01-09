package leetcode

func subtreeWithAllDeepest(root *TreeNode) (ans *TreeNode) {
	maxDepth := -1

	var dfs func(*TreeNode, int) int

	dfs = func(node *TreeNode, depth int) int {
		if node == nil {
			maxDepth = max(maxDepth, depth)
			return depth
		}

		leftDepth := dfs(node.Left, depth+1)
		rightDepth := dfs(node.Right, depth+1)

		if leftDepth == rightDepth && leftDepth == maxDepth {
			ans = node
		}

		return max(leftDepth, rightDepth)
	}

	dfs(root, 0)
	return
}
