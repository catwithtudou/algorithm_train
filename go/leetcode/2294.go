package leetcode

import (
	"math"
	"slices"
)

func partitionArray(nums []int, k int) (ans int) {
	slices.Sort(nums)
	mn := math.MinInt / 2
	for _, x := range nums {
		if x-mn > k {
			ans++
			mn = x
		}
	}
	return
}
