package leetcode

import "math"

func paintWalls(cost []int, time []int) int {
	n := len(cost)
	memo := make([][]int, n)
	for i := range memo {
		memo[i] = make([]int, n*2+1)
		for j := range memo[i] {
			memo[i][j] = -1
		}
	}
	var dfs func(int, int) int
	dfs = func(i int, j int) int {
		if j > i {
			return 0
		}

		if i < 0 {
			return math.MaxInt / 2
		}

		p := &memo[i][j+n]
		if *p != -1 {
			return *p
		}
		*p = min(dfs(i-1, j+time[i])+cost[i], dfs(i-1, j-1))
		return *p
	}
	return dfs(n-1, 0)
}
