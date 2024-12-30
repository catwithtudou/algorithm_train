package leetcode

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func isSubPath(head *ListNode, root *TreeNode) bool {
	var dfs func(*ListNode, *TreeNode) bool
	dfs = func(l *ListNode, r *TreeNode) bool {
		if l == nil {
			return true
		}
		if r == nil {
			return false
		}
		return l.Val == r.Val && (dfs(l.Next, r.Left) || dfs(l.Next, r.Right)) ||
			l == head && (dfs(head, r.Left) || dfs(head, r.Right))
	}
	return dfs(head, root)
}
