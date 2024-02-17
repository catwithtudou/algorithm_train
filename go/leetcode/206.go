package leetcode

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func reverseList(head *ListNode) *ListNode {
	var cur, pre, tmp *ListNode
	cur = head
	for cur != nil {
		tmp = cur.Next
		cur.Next = pre
		pre = cur
		cur = tmp
	}
	return pre
}

func reverseListOther(head *ListNode) *ListNode {
	var reverse func(*ListNode, *ListNode) *ListNode
	reverse = func(pre *ListNode, cur *ListNode) *ListNode {
		if cur == nil {
			return pre
		}
		temp := cur.Next
		cur.Next = pre
		return reverse(cur, temp)
	}

	return reverse(nil, head)
}
