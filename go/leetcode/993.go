package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func isCousins(root *TreeNode, x int, y int) bool {
	var xParent, yParent *TreeNode
	var xDepth, yDepth int
	var xFound, yFound bool

	var dfs func(node, parent *TreeNode, depth int)
	dfs = func(node, parent *TreeNode, depth int) {
		if node == nil {
			return
		}

		if node.Val == x {
			xParent, xDepth, xFound = parent, depth, true
		} else if node.Val == y {
			yParent, yDepth, yFound = parent, depth, true
		}

		if xFound && yFound {
			return
		}

		dfs(node.Left, node, depth+1)

		if xFound && yFound {
			return
		}

		dfs(node.Right, node, depth+1)
	}
	dfs(root, nil, 0)

	return xDepth == yDepth && xParent != yParent
}

func isCousinsBFS(root *TreeNode, x int, y int) bool {
	var xParent, yParent *TreeNode
	var xDepth, yDepth int
	var xFound, yFound bool

	update := func(node, parent *TreeNode, depth int) {
		if node.Val == x {
			xParent, xDepth, xFound = parent, depth, true
		} else if node.Val == y {
			yParent, yDepth, yFound = parent, depth, true
		}
	}

	type pair struct {
		node  *TreeNode
		depth int
	}
	q := []pair{{root, 0}}
	update(root, nil, 0)
	for len(q) > 0 && (!xFound || !yFound) {
		node, depth := q[0].node, q[0].depth
		q = q[1:]
		if node.Left != nil {
			q = append(q, pair{node.Left, depth + 1})
			update(node.Left, node, depth+1)
		}
		if node.Right != nil {
			q = append(q, pair{node.Right, depth + 1})
			update(node.Right, node, depth+1)
		}
	}

	return xDepth == yDepth && xParent != yParent
}
