package leetcode

import "math"

func minimumTotal(triangle [][]int) int {
	n := len(triangle)
	memo := make([][]int, n)
	for i := range memo {
		memo[i] = make([]int, n)
		for j := range memo[i] {
			memo[i][j] = math.MinInt
		}
	}

	var dfs func(int, int) int

	dfs = func(i, j int) int {
		if i == n-1 {
			return triangle[i][j]
		}
		p := &memo[i][j]
		if *p != math.MinInt {
			return *p
		}
		*p = min(dfs(i+1, j), dfs(i+1, j+1)) + triangle[i][j]
		return *p
	}

	return dfs(0, 0)
}
