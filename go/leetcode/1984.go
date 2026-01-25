package leetcode

import (
	"math"
	"slices"
)

func minimumDifference1984(nums []int, k int) int {
	slices.Sort(nums)
	ans := math.MaxInt
	for i := k - 1; i < len(nums); i++ {
		ans = min(ans, nums[i]-nums[i-k+1])
	}
	return ans
}
