package leetcode

import "math"

func minimumSubstringsInPartition(s string) int {
	n := len(s)
	memo := make([]int, n)
	var dfs func(int) int
	dfs = func(i int) int {
		if i < 0 {
			return 0
		}
		p := &memo[i]
		if *p > 0 {
			return *p
		}
		res, k, maxCnt := math.MaxInt, 0, 0
		cnt := [26]int{}
		for j := i; j >= 0; j-- {
			c := s[j] - 'a'
			if cnt[c] == 0 {
				k++
			}
			cnt[c]++
			maxCnt = max(maxCnt, cnt[c])
			if i-j+1 == k*maxCnt {
				res = min(res, dfs(j-1)+1)
			}
		}
		*p = res
		return res
	}

	return dfs(n - 1)
}
