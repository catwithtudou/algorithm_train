package leetcode

import "golang.org/x/exp/slices"

func maxTotalValue(nums []int, k int) int64 {
	return int64(slices.Max(nums)-slices.Min(nums)) * int64(k)
}
