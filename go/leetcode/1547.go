package leetcode

import (
	"math"

	"golang.org/x/exp/slices"
)

func minCost1547(n int, cuts []int) int {
	cuts = append(cuts, 0, n)
	slices.Sort(cuts)

	m := len(cuts)
	memo := make([][]int, m)
	for i := range memo {
		memo[i] = make([]int, m)
	}
	var dfs func(int, int) int
	dfs = func(i, j int) int {
		if i+1 == j {
			return 0
		}
		p := &memo[i][j]
		if *p != 0 {
			return *p
		}
		res := math.MaxInt
		for k := i + 1; k < j; k++ {
			res = min(res, dfs(i, k)+dfs(k, j))
		}
		*p = res + cuts[j] - cuts[i]
		return *p
	}

	return dfs(0, m-1)
}
