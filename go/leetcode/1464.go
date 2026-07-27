package leetcode

import "slices"

func maxProduct1464(nums []int) int {
	slices.Sort(nums)
	n := len(nums)
	return (nums[n-1] - 1) * (nums[n-2] - 1)
}
