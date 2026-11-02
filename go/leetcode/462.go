package leetcode

import "golang.org/x/exp/slices"

func minMoves2(nums []int) (ans int) {
	slices.Sort(nums)
	medium := nums[len(nums)/2]

	for _, x := range nums {
		ans += abs(x - medium)
	}

	return
}
