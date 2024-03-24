package leetcode

import "math"

func coinChange(coins []int, amount int) int {
	n := len(coins)
	cache := make([][]int, n)
	for c := range cache {
		cache[c] = make([]int, amount+1)
		for i := range cache[c] {
			cache[c][i] = -1
		}
	}

	var dfs func(i, count int) (res int)
	dfs = func(i, count int) (res int) {
		if i < 0 {
			if count == 0 {
				return 0
			}
			return math.MaxInt / 2
		}

		c := &cache[i][count]
		if *c != -1 {
			return *c
		}
		defer func() { *c = res }()

		if coins[i] > count {
			return dfs(i-1, count)
		}

		return min(dfs(i-1, count), dfs(i, count-coins[i])+1)
	}

	ans := dfs(n-1, amount)
	if ans < math.MaxInt/2 {
		return ans
	}

	return -1
}
