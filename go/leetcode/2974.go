package leetcode

import "golang.org/x/exp/slices"

func numberGame(nums []int) []int {
	slices.Sort(nums)
	for i := len(nums) - 1; i >= 1; i -= 2 {
		nums[i], nums[i-1] = nums[i-1], nums[i]
	}
	return nums
}
