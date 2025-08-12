package leetcode

import "math"

func numberOfWays(n int, x int) int {
	dp := make([]int, n+1)
	dp[0] = 1

	for i := 1; powByMath(i, x) <= n; i++ {
		v := powByMath(i, x)
		for j := n; j >= v; j-- {
			dp[j] += dp[j-v]
		}
	}

	return dp[n] % (1e9 + 7)
}

func powByMath(i, x int) int {
	return int(math.Pow(float64(i), float64(x)))
}
