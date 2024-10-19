package leetcode

func minOperations3192(nums []int) int {
	ans := 0
	for _, v := range nums {
		if v == ans%2 {
			ans++
		}
	}
	return ans
}
