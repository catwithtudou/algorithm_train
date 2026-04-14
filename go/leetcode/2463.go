package leetcode

import (
	"math"

	"golang.org/x/exp/slices"
)

func minimumTotalDistance(robot []int, factory [][]int) int64 {
	slices.SortFunc(factory, func(a, b []int) int { return a[0] - b[0] })
	slices.Sort(robot)

	n, m := len(factory), len(robot)
	memo := make([][]int, n)
	for i := range memo {
		memo[i] = make([]int, m)
		for j := range memo[i] {
			memo[i][j] = -1
		}
	}

	var dfs func(int, int) int

	dfs = func(i, j int) (res int) {
		if j < 0 {
			return 0
		}

		if i < 0 {
			return math.MaxInt / 2
		}

		p := &memo[i][j]
		if *p != -1 {
			return *p
		}
		defer func() { *p = res }()

		res = dfs(i-1, j)

		po, limit := factory[i][0], factory[i][1]
		disSum := 0
		for k := 1; k <= min(j+1, limit); k++ {
			disSum += abs(robot[j-k+1] - po)
			res = min(res, dfs(i-1, j-k)+disSum)
		}
		return
	}

	return int64(dfs(n-1, m-1))
}
