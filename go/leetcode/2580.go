package leetcode

import "golang.org/x/exp/slices"

func countWays(ranges [][]int) int {
	slices.SortFunc(ranges, func(a, b []int) int {
		return a[0] - b[0]
	})
	ans, maxR := 1, -1
	for _, v := range ranges {
		if v[0] > maxR {
			ans = ans * 2 % 1_000_000_007
		}
		maxR = max(maxR, v[1])
	}
	return ans
}
