package leetcode

import "slices"

func minimumCost2144(cost []int) int {
	slices.Sort(cost)
	ans := 0
	for i := len(cost) - 1; i >= 0; i -= 3 {
		ans += cost[i]
		if i > 0 {
			ans += cost[i-1]
		}
	}
	return ans
}
