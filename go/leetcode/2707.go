package leetcode

func minExtraChar(s string, dictionary []string) int {
	dMap := make(map[string]bool)
	for _, v := range dictionary {
		dMap[v] = true
	}
	sLen := len(s)
	dp := make([]int, sLen+1)
	for i := 1; i <= sLen; i++ {
		dp[i] = dp[i-1] + 1
		for j := 0; j < i; j++ {
			if dMap[s[j:i]] && dp[j] < dp[i] {
				dp[i] = dp[j]
			}
		}

	}
	return dp[sLen]
}
