package leetcode

func change(amount int, coins []int) int {
	dp := make([]int, amount+1)
	dp[0] = 1
	for _, v := range coins {
		for i := 1; i <= amount; i++ {
			if v > i {
				continue
			}
			dp[i] += dp[i-v]
		}
	}

	return dp[amount]
}
