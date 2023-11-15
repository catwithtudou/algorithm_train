package leetcode

import "golang.org/x/exp/slices"

func maximizeSum(nums []int, k int) int {
	return slices.Max(nums)*k + (k-1)*k/2
}
