package leetcode

import "golang.org/x/exp/slices"

func semiOrderedPermutation(nums []int) int {
	minIdx := slices.Index(nums, 1)
	maxIdx := slices.Index(nums, len(nums))
	ans := minIdx + len(nums) - maxIdx - 1
	if maxIdx >= minIdx {
		return ans
	}
	return ans - 1
}
