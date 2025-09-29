package leetcode

import "math"

func minScoreTriangulation(values []int) int {
	n := len(values)
	memo := make([][]int, n)
	for i := range memo {
		memo[i] = make([]int, n)
		for j := range memo[i] {
			memo[i][j] = -1
		}
	}

	var dfs func(int, int) int

	dfs = func(i int, j int) int {
		if i+1 == j {
			return 0
		}

		p := &memo[i][j]
		if *p != -1 {
			return *p
		}

		res := math.MaxInt
		for k := i + 1; k < j; k++ {
			res = min(res, dfs(i, k)+dfs(k, j)+values[i]*values[k]*values[j])
		}
		*p = res
		return res
	}

	return dfs(0, n-1)
}
