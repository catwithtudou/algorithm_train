package leetcode

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val   int
 *     Left  *TreeNode
 *     Right *TreeNode
 * }
 */

func getPath(node *TreeNode, target *TreeNode) []*TreeNode {
	var ans []*TreeNode
	for node != target {
		ans = append(ans, node)
		if node.Val > target.Val {
			node = node.Left
		} else {
			node = node.Right
		}
	}
	ans = append(ans, node)
	return ans
}

func lowestCommonAncestor236(root, p, q *TreeNode) *TreeNode {
	left := getPath(root, p)
	right := getPath(root, q)
	var ans *TreeNode
	for i := 0; i < len(left) && i < len(right) && left[i] == right[i]; i++ {
		ans = left[i]
	}
	return ans
}

func lowestCommonAncestor236Other(root, p, q *TreeNode) *TreeNode {
	ans := root
	for {

		if ans.Val < q.Val && ans.Val < p.Val {
			ans = ans.Right
			continue
		} else if ans.Val > q.Val && ans.Val > p.Val {
			ans = ans.Left
			continue
		}
		break
	}

	return ans
}
