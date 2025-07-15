package leetcode

/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func getDecimalValue(head *ListNode) (ans int) {
	for head != nil {
		ans = ans*2 + head.Val
		head = head.Next
	}
	return
}
