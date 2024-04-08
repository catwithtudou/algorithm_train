package leetcode

import "golang.org/x/exp/slices"

func minOperations(nums []int) int {
	n := len(nums)
	slices.Sort(nums)
	reNums := slices.Compact(nums)
	ans, left := 0, 0
	for i, x := range reNums {
		for reNums[left] < x-n+1 {
			left++
		}
		ans = max(ans, i-left+1)
	}
	return n - ans
}
