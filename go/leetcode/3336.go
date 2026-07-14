package leetcode

import "golang.org/x/exp/slices"

func subsequencePairCount(nums []int) int {
	n := len(nums)
	m := slices.Max(nums)
	memo := make([][][]int, n)
	for i := range memo {
		memo[i] = make([][]int, m+1)
		for j := range memo[i] {
			memo[i][j] = make([]int, m+1)
			for k := range memo[i][j] {
				memo[i][j][k] = -1
			}
		}
	}

	var dfs func(int, int, int) int

	dfs = func(i, j, k int) int {

		if i < 0 {
			if j == k {
				return 1
			}
			return 0
		}

		p := &memo[i][j][k]
		if *p < 0 {
			*p = (dfs(i-1, j, k) + dfs(i-1, gcd(j, nums[i]), k) + dfs(i-1, j, gcd(k, nums[i]))) % mod
		}

		return *p
	}

	return (dfs(n-1, 0, 0) - 1 + mod) % mod
}
