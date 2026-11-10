package leetcode

import "math"

func climbStairsII(n int, costs []int) int {
	memo := make([]int, n+1)
	var dfs func(int) int

	dfs = func(i int) int {
		if i == 0 {
			return 0
		}

		p := &memo[i]
		if *p != 0 {
			return *p
		}

		res := math.MaxInt
		for j := max(i-3, 0); j < i; j++ {
			res = min(res, dfs(j)+(i-j)*(i-j))
		}
		res += costs[i-1]
		*p = res
		return res
	}

	return dfs(n)
}
