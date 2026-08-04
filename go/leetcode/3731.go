package leetcode

import "math"

func findMissingElements(nums []int) (ans []int) {
	mn, mx := math.MaxInt, math.MinInt

	has := map[int]bool{}

	for _, x := range nums {
		mn = min(mn, x)
		mx = max(mx, x)
		has[x] = true
	}

	for i := mn + 1; i < mx; i++ {
		if !has[i] {
			ans = append(ans, i)
		}
	}

	return
}
