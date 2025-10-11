package leetcode

import (
	"slices"
)

func maximumTotalDamage(power []int) int64 {
	cnt := make(map[int]int)
	for _, v := range power {
		cnt[v]++
	}

	n := len(cnt)
	num := make([]int, 0, n)
	for k := range cnt {
		num = append(num, k)
	}
	slices.Sort(num)

	memo := make([]int, n)
	for i := range memo {
		memo[i] = -1
	}

	var dfs func(int) int

	dfs = func(i int) int {
		if i < 0 {
			return 0
		}

		p := &memo[i]
		if *p != -1 {
			return *p
		}

		x := num[i]
		j := i
		for j > 0 && num[j-1] >= x-2 {
			j--
		}

		*p = max(dfs(i-1), dfs(j-1)+x*cnt[x])
		return *p
	}

	return int64(dfs(n - 1))
}
