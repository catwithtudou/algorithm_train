package leetcode

import (
	"math"

	"golang.org/x/exp/slices"
)

func findValueOfPartition(nums []int) int {
	ans := math.MaxInt
	slices.Sort(nums)
	for i := 1; i < len(nums); i++ {
		ans = min(ans, nums[i]-nums[i-1])
	}
	return ans
}
