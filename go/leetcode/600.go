package leetcode

import "math/bits"

func findIntegers(n int) int {
	m := bits.Len(uint(n))
	memo := make([][2]int, m)
	for i := range memo {
		memo[i] = [2]int{-1, -1}
	}
	var dfs func(int, int, bool) int
	dfs = func(i int, pre int, isLimit bool) (res int) {
		if i < 0 {
			return 1
		}

		if !isLimit {
			p := &memo[i][pre]
			if *p >= 0 {
				return *p
			}
			defer func() { *p = res }()
		}

		up := 1
		if isLimit {
			up = n >> i & 1
		}
		res = dfs(i-1, 0, isLimit && up == 0)
		if pre == 0 && up == 1 {
			res += dfs(i-1, 1, isLimit)
		}
		return
	}

	return dfs(m-1, 0, true)
}
