package leetcode

import "math"

func minimumValueSum(nums []int, andValues []int) int {
	n, m := len(nums), len(andValues)
	const inf = math.MaxInt / 2
	type pair struct{ i, j, and int }
	memo := make(map[pair]int, 0)
	var dfs func(i, j, and int) int
	dfs = func(i, j, and int) int {
		if n-i < m-j {
			return inf
		}
		if j == m {
			if i == n {
				return 0
			}
			return inf
		}
		and &= nums[i]
		pair := pair{i, j, and}
		if v, ok := memo[pair]; ok {
			return v
		}
		res := dfs(i+1, j, and)
		if and == andValues[j] {
			res = min(res, dfs(i+1, j+1, -1)+nums[i])
		}
		memo[pair] = res
		return res
	}

	and := dfs(0, 0, -1)
	if and == inf {
		return -1
	}

	return and
}
