package leetcode

func deleteDuplicates(head *ListNode) *ListNode {
	if head == nil {
		return head
	}
	node := head
	for node.Next != nil {
		if node.Val != node.Next.Val {
			node = node.Next
			continue
		}
		node.Next = node.Next.Next
	}

	return head
}
