package leetcode

import "math"

func maxProductPath(grid [][]int) int {

	m, n := len(grid), len(grid[0])

	memo := make([][][2]int, m)
	for i := range memo {
		memo[i] = make([][2]int, n)
		for j := range memo[i] {
			memo[i][j] = [2]int{math.MinInt, math.MinInt}
		}
	}

	var dfs func(int, int) (int, int)
	dfs = func(i, j int) (int, int) {
		x := grid[i][j]
		if i == 0 && j == 0 {
			return x, x
		}

		p := &memo[i][j]
		if p[0] != math.MinInt {
			return p[0], p[1]
		}

		resMin := math.MaxInt
		resMax := math.MinInt

		if i > 0 {
			mn, mx := dfs(i-1, j)
			resMin = min(mx*x, mn*x)
			resMax = max(mn*x, mx*x)
		}

		if j > 0 {
			mn, mx := dfs(i, j-1)
			resMin = min(resMin, min(mx*x, mn*x))
			resMax = max(resMax, max(mx*x, mn*x))
		}

		p[0] = resMin
		p[1] = resMax

		return resMin, resMax
	}

	_, ans := dfs(m-1, n-1)
	if ans < 0 {
		return -1
	}

	return ans % int(1e9+7)
}
