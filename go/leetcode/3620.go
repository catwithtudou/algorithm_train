package leetcode

import (
	"math"
	"sort"
)

func findMaxPathScore(edges [][]int, online []bool, k int64) int {

	n := len(online)

	type edge struct{ to, wt int }

	g := make([][]edge, n)
	maxWt := -1
	for _, e := range edges {
		a, b, wt := e[0], e[1], e[2]
		if online[a] && online[b] {
			g[a] = append(g[a], edge{b, wt})
			if a == 0 {
				maxWt = max(maxWt, wt)
			}
		}
	}

	memo := make([]int, n)

	ans := sort.Search(maxWt+1, func(lower int) bool {

		for i := range memo {
			memo[i] = -1
		}

		var dfs func(int) int

		dfs = func(i int) int {

			if i == n-1 {
				return 0
			}

			p := &memo[i]
			if *p != -1 {
				return *p
			}

			res := math.MaxInt / 2

			for _, e := range g[i] {
				y := e.to
				if e.wt >= lower {
					res = min(res, dfs(y)+e.wt)
				}

			}
			*p = res
			return res
		}

		return dfs(0) > int(k)
	}) - 1

	return ans
}
