package leetcode

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func insertGreatestCommonDivisors(head *ListNode) *ListNode {
	node := head
	for node.Next != nil {
		node.Next = &ListNode{
			Val:  gcb(node.Val, node.Next.Val),
			Next: node.Next,
		}
		node = node.Next.Next
	}

	return head
}

func gcb(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}
