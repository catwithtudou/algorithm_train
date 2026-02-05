package leetcode

func constructTransformedArray(nums []int) []int {
	n := len(nums)
	result := make([]int, n)
	for i, x := range nums {
		result[i] = nums[((i+x)%n+n)%n]
	}
	return result
}
