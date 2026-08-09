package leetcode

import "math"

func stoneGameII(piles []int) int {
	n := len(piles)

	memo := make([][]int, n-1)
	for i := range memo {
		memo[i] = make([]int, (n+1)/4+1)
		for j := range memo[i] {
			memo[i][j] = -1
		}
	}

	for i := n - 2; i >= 0; i-- {
		piles[i] += piles[i+1]
	}

	var dfs func(int, int) int

	dfs = func(i, m int) int {
		if i+2*m >= n {
			return piles[i]
		}

		p := &memo[i][m]
		if *p != -1 {
			return *p
		}

		mn := math.MaxInt
		for x := 1; x <= 2*m; x++ {
			mn = min(mn, dfs(i+x, max(m, x)))
		}
		*p = piles[i] - mn
		return *p
	}

	return dfs(0, 1)
}
