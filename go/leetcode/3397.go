package leetcode

import (
	"math"

	"golang.org/x/exp/slices"
)

func maxDistinctElements(nums []int, k int) (ans int) {
	slices.Sort(nums)
	pre := math.MinInt
	for _, x := range nums {
		x = min(max(x-k, pre+1), x+k)
		if x > pre {
			ans++
			pre = x
		}
	}
	return
}
