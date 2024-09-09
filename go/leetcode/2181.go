package leetcode

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func mergeNodes(head *ListNode) *ListNode {
	tail := head

	for cur := head.Next; cur.Next != nil; cur = cur.Next {
		if cur.Val != 0 {
			tail.Val += cur.Val
		} else {
			tail = tail.Next
			tail.Val = 0
		}
	}
	tail.Next = nil

	return head
}
