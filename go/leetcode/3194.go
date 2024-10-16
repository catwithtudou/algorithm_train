package leetcode

import (
	"math"

	"golang.org/x/exp/slices"
)

func minimumAverage(nums []int) float64 {
	slices.Sort(nums)
	ans := math.MaxInt64
	for i, n := 0, len(nums); i < n/2; i++ {
		ans = min(ans, nums[i]+nums[n-1-i])
	}
	return float64(ans) / 2
}
