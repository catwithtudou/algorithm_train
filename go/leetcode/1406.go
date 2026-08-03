package leetcode

import "math"

func stoneGameIII(stoneValue []int) string {
	n := len(stoneValue)
	memo := make([]int, n)
	for i := range memo {
		memo[i] = math.MinInt
	}

	var dfs func(int) int
	dfs = func(i int) int {
		if i == n {
			return 0
		}

		p := &memo[i]
		if *p != math.MinInt {
			return *p
		}

		res := math.MinInt
		sum := 0
		for j := i; j < min(i+3, n); j++ {
			sum += stoneValue[j]
			res = max(res, sum-dfs(j+1))
		}
		*p = res
		return res
	}

	diff := dfs(0)
	if diff == 0 {
		return "Tie"
	}
	if diff > 0 {
		return "Alice"
	}
	return "Bob"
}
