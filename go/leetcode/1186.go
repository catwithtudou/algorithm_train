package leetcode

import "math"

func maximumSum(arr []int) int {
	memo := make([][2]int, len(arr))
	for i := range arr {
		memo[i] = [2]int{math.MinInt, math.MinInt}
	}
	var dfs func(i, j int) int
	dfs = func(i, j int) (res int) {
		if i < 0 {
			return math.MinInt / 2
		}
		p := &memo[i][j]
		if *p != math.MinInt {
			return *p
		}
		defer func() { *p = res }()
		if j == 0 {
			return max(dfs(i-1, 0), 0) + arr[i]
		}
		return max(dfs(i-1, 1)+arr[i], dfs(i-1, 0))
	}
	ans := math.MinInt
	for i := range arr {
		ans = max(ans, dfs(i, 0))
		ans = max(ans, dfs(i, 1))
	}
	return ans
}
