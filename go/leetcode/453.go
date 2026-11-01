package leetcode

import "golang.org/x/exp/slices"

func minMoves453(nums []int) (ans int) {
	for _, x := range nums {
		ans += x
	}
	return ans - slices.Min(nums)*len(nums)
}
