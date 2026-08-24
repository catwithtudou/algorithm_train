package leetcode

import "math"

func stoneGameVIII(stones []int) int {
	n := len(stones)
	sum := make([]int, n)
	sum[0] = stones[0]
	for i := 1; i < n; i++ {
		sum[i] = sum[i-1] + stones[i]
	}

	memo := make([]int, n-1)
	for i := range memo {
		memo[i] = math.MaxInt
	}

	var dfs func(int) int
	dfs = func(i int) int {
		if i == n-1 {
			return sum[n-1]
		}

		p := &memo[i]
		if *p == math.MaxInt {
			*p = max(dfs(i+1), sum[i]-dfs(i+1))
		}
		return *p
	}

	return dfs(1)
}
