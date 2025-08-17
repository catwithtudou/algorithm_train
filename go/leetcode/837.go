package leetcode

func new21Game(n int, k int, maxPts int) float64 {
	if k == 0 {
		return 1.0
	}

	dp := make([]float64, k+maxPts)

	sum := 0.0

	for i := k + maxPts - 1; i >= k; i-- {
		if i <= n {
			dp[i] = 1.0
		}
		sum += dp[i]
	}

	for i := k - 1; i >= 0; i-- {
		dp[i] = sum / float64(maxPts)
		sum += dp[i] - dp[i+maxPts]
	}

	return dp[0]
}
