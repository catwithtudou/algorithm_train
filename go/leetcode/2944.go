package leetcode

import "math"

func minimumCoins(prices []int) int {
	n := len(prices)
	memo := make([]int, (n+1)/2)
	var dfs func(int) int
	dfs = func(i int) (res int) {
		if 2*i >= n {
			return prices[i-1]
		}
		p := &memo[i]
		if *p > 0 {
			return *p
		}
		defer func() { *p = res }()

		res = math.MaxInt
		for j := i + 1; j <= 2*i+1; j++ {
			res = min(res, dfs(j))
		}
		return res + prices[i-1]
	}
	return dfs(1)
}
