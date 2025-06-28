package leetcode

import "slices"

func maxSubsequence(nums []int, k int) []int {
	idx := make([]int, len(nums))
	for i := range idx {
		idx[i] = i
	}

	slices.SortFunc(idx, func(i, j int) int {
		return nums[j] - nums[i]
	})

	idx = idx[:k]
	slices.Sort(idx)

	for i, j := range idx {
		idx[i] = nums[j]
	}

	return idx
}
