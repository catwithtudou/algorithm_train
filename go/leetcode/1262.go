package leetcode

import "math"

func maxSumDivThree(nums []int) int {
	n := len(nums)
	memo := make([][3]int, n)
	for i := range memo {
		for j := range memo[i] {
			memo[i][j] = -1
		}
	}

	var dfs func(int, int) int

	dfs = func(i, j int) int {
		if i < 0 {
			if j == 0 {
				return 0
			}
			return math.MinInt32
		}
		if memo[i][j] != -1 {
			return memo[i][j]
		}

		res := max(dfs(i-1, j), dfs(i-1, (j+nums[i])%3)+nums[i])
		memo[i][j] = res
		return res
	}

	return dfs(n-1, 0)
}
