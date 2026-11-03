package leetcode

import "golang.org/x/exp/slices"

func minMoves3736(nums []int) (ans int) {
	ans = slices.Max(nums) * len(nums)
	for _, x := range nums {
		ans -= x
	}
	return
}
