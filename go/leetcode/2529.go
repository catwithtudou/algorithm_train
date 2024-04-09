package leetcode

import "sort"

func maximumCount(nums []int) int {
	neg := sort.SearchInts(nums, 0)
	pos := len(nums) - sort.SearchInts(nums, 1)
	return max(neg, pos)
}
