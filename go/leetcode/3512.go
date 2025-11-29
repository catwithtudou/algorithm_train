package leetcode

func minOperations3512(nums []int, k int) int {
	sum := 0
	for _, x := range nums {
		sum += x
	}
	return sum % k
}
