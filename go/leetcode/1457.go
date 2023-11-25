package leetcode

/**
 * Definition for a binary tree node.

 * }
 */

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func pseudoPalindromicPaths(root *TreeNode) int {
	return pseudoPalindromicPathsDfs(root, 0)
}

func pseudoPalindromicPathsDfs(root *TreeNode, mask int) int {
	if root == nil {
		return 0
	}

	mask ^= 1 << root.Val
	if root.Left == root.Right { // 叶子结点
		if mask&(mask-1) == 0 {
			return 1
		}
		return 0
	}

	return pseudoPalindromicPathsDfs(root.Left, mask) + pseudoPalindromicPathsDfs(root.Right, mask)
}
