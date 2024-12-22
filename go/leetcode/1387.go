package leetcode

import (
	"cmp"
	"slices"
)

func getKth(lo int, hi int, k int) int {
	memo := make(map[int]int, 0)

	var dfs func(int) int
	dfs = func(i int) int {
		if i == 1 {
			return 0
		}
		if res, ok := memo[i]; ok {
			return res
		}
		if i%2 == 0 {
			memo[i] = dfs(i/2) + 1
		} else {
			memo[i] = dfs((i*3+1)/2) + 2
		}
		return memo[i]
	}

	nums := make([]int, hi-lo+1)
	for i := range nums {
		nums[i] = lo + i
	}

	slices.SortFunc(nums, func(x, y int) int {
		return cmp.Or(dfs(x)-dfs(y), x-y)
	})
	return nums[k-1]
}
