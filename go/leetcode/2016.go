package leetcode

import "math"

func maximumDifference(nums []int) (ans int) {
	preMin := math.MaxInt
	for _, x := range nums {
		ans = max(ans, x-preMin)
		preMin = min(preMin, x)
	}

	if ans == 0 {
		return -1
	}

	return
}
