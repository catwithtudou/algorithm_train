package leetcode

import "slices"

func minRemoval(nums []int, k int) int {
	slices.Sort(nums)
	maxSave, left := 0, 0

	for i, mx := range nums {
		for nums[left]*k < mx {
			left++
		}
		maxSave = max(maxSave, i-left+1)
	}

	return len(nums) - maxSave
}
