package leetcode

import "math"

func minPathCost(grid [][]int, moveCost [][]int) int {
	m, n := len(grid), len(grid[0])
	memo := make([][]int, m)
	for i := range memo {
		memo[i] = make([]int, n)
	}
	var dfs func(i, j int) int
	dfs = func(i, j int) int {
		if i == m-1 {
			return grid[i][j]
		}

		p := &memo[i][j]
		if *p != 0 {
			return *p
		}

		res := math.MaxInt
		for k, c := range moveCost[grid[i][j]] {
			res = min(res, dfs(i+1, k)+c)
		}
		*p = res + grid[i][j]
		return *p
	}

	ans := math.MaxInt
	for j := 0; j < n; j++ {
		ans = min(ans, dfs(0, j))
	}

	return ans
}
