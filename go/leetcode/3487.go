package leetcode

import "math"

func maxSum(nums []int) (ans int) {
	set := make(map[int]bool)
	mx := math.MinInt
	for _, x := range nums {
		if x < 0 {
			mx = max(mx, x)
		} else if _, ok := set[x]; !ok {
			ans += x
			set[x] = true
		}
	}

	if len(set) == 0 {
		return mx
	}

	return
}
