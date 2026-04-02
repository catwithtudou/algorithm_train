package leetcode

import "math"

func maximumAmount(coins [][]int) int {
	m, n := len(coins), len(coins[0])
	memo := make([][][3]int, m)
	for i := range memo {
		memo[i] = make([][3]int, n)
		for j := range memo[i] {
			for k := range memo[i][j] {
				memo[i][j][k] = math.MinInt
			}
		}
	}

	var dfs func(int, int, int) int

	dfs = func(i, j, k int) int {
		if i < 0 || j < 0 {
			return math.MinInt
		}

		x := coins[i][j]
		if i == 0 && j == 0 {
			if k == 0 {
				return x
			}
			return max(x, 0)
		}

		p := &memo[i][j][k]
		if *p != math.MinInt {
			return *p
		}

		res := max(dfs(i-1, j, k), dfs(i, j-1, k)) + x
		if k > 0 && x < 0 {
			res = max(res, max(dfs(i-1, j, k-1), dfs(i, j-1, k-1)))
		}

		*p = res
		return res
	}

	return dfs(m-1, n-1, 2)
}
