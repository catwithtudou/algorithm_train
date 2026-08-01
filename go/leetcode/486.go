package leetcode

import "math"

func predictTheWinner(nums []int) bool {
	n := len(nums)

	memo := make([][]int, n)
	for i := range memo {
		memo[i] = make([]int, n)
		for j := range memo[i] {
			memo[i][j] = math.MinInt
		}
	}

	var dfs func(int, int) int
	dfs = func(i, j int) (res int) {
		p := &memo[i][j]
		if *p != math.MinInt {
			return *p
		}
		defer func() {
			*p = res
		}()

		if i == j {
			return nums[i]
		}

		return max(nums[i]-dfs(i+1, j), nums[j]-dfs(i, j-1))
	}

	return dfs(0, n-1) >= 0
}
