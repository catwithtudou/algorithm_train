package leetcode

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func swapPairs(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}
	dummyHead := &ListNode{0, nil}
	dummyHead.Next = head
	cur := dummyHead
	for cur.Next != nil && cur.Next.Next != nil {
		tmp := cur.Next
		tmp1 := cur.Next.Next.Next

		cur.Next = cur.Next.Next
		cur.Next.Next = tmp
		cur.Next.Next.Next = tmp1

		cur = cur.Next.Next
	}
	return dummyHead.Next
}

func swapPairsOther(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}
	next := head.Next
	head.Next = swapPairsOther(next.Next)
	next.Next = head
	return next
}
