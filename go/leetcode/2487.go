package leetcode

type ListNode struct {
	Val  int
	Next *ListNode
}

// 递归
func removeNodes(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}
	head.Next = removeNodes(head.Next)
	if head.Next != nil && head.Next.Val > head.Val {
		return head.Next
	}
	return head
}

// 单调栈
func removeNodesByStack(head *ListNode) *ListNode {
	stackNode := make([]*ListNode, 0)
	for ; head != nil; head = head.Next {
		stackNode = append(stackNode, head)
	}

	for ; len(stackNode) > 0; stackNode = stackNode[:len(stackNode)-1] {
		if head == nil || stackNode[len(stackNode)-1].Val >= head.Val {
			stackNode[len(stackNode)-1].Next = head
			head = stackNode[len(stackNode)-1]
		}
	}

	return head
}
