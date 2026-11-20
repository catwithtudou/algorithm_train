package leetcode

import "slices"

func minimizeSum(nums []int) int {
	slices.Sort(nums)
	n := len(nums)
	return min(min(nums[n-3]-nums[0], nums[n-2]-nums[1]), nums[n-1]-nums[2])
}
