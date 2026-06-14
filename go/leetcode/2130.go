package leetcode

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func pairSum(head *ListNode) (ans int) {
	left := head

	var dfs func(*ListNode)

	dfs = func(right *ListNode) {
		if right.Next != nil {
			dfs(right.Next)
		}

		ans = max(ans, left.Val+right.Val)
		left = left.Next
	}

	dfs(head)
	return
}
