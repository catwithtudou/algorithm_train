package leetcode

func modifiedList(nums []int, head *ListNode) *ListNode {
	has := make(map[int]bool)
	for _, num := range nums {
		has[num] = true
	}

	dummy := &ListNode{Next: head}
	cur := dummy
	for cur.Next != nil {
		nxt := cur.Next
		if has[nxt.Val] {
			cur.Next = nxt.Next
		} else {
			cur = nxt
		}
	}

	return dummy.Next
}
