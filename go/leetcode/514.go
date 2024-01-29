package leetcode

import "math"

func findRotateSteps(ring string, key string) int {
	const inf = math.MaxInt64 / 2
	n, m := len(ring), len(key)
	pos := [26][]int{}
	for i, v := range ring {
		pos[v-'a'] = append(pos[v-'a'], i)
	}

	dp := make([][]int, m)
	for i := range dp {
		dp[i] = make([]int, n)
		for j := range ring {
			dp[i][j] = inf
		}
	}

	for _, v := range pos[key[0]-'a'] {
		dp[0][v] = 1 + min(n-v, v)
	}

	for i := 1; i < m; i++ {
		for _, j := range pos[key[i]-'a'] {
			for _, v := range pos[key[i-1]-'a'] {
				dp[i][j] = min(dp[i][j], dp[i-1][v]+min(n-absInt(j-v), absInt(j-v))+1)
			}
		}
	}

	return minSlice(dp[m-1]...)
}

func minSlice(a ...int) int {
	if len(a) == 0 {
		return 0
	}
	ans := a[0]
	for _, v := range a {
		if v < ans {
			ans = v
		}
	}

	return ans
}

func absInt(x int) int {
	if x < 0 {
		return -x
	}
	return x
}
