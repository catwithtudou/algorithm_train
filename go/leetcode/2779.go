package leetcode

import (
	"golang.org/x/exp/slices"
)

func maximumBeauty(nums []int, k int) int {
	slices.Sort(nums)
	l := 0
	ans := 0
	for r, x := range nums {
		if x-nums[l] > 2*k {
			l++
		}
		ans = max(ans, r-l+1)
	}
	return ans
}
