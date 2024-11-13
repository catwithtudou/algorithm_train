package leetcode

import "sort"

func countKConstraintSubstringsII(s string, k int, queries [][]int) []int64 {
	n := len(s)
	left := make([]int, n)
	sum := make([]int, n+1)
	cnt := [2]int{}
	l := 0
	for i, c := range s {
		cnt[c&1]++
		for cnt[0] > k && cnt[1] > k {
			cnt[s[l]&1]--
			l++
		}
		left[i] = l
		sum[i+1] = sum[i] + i - l + 1
	}

	ans := make([]int64, len(queries))
	for i, query := range queries {
		l, r := query[0], query[1]
		j := l + sort.SearchInts(left[l:r+1], l)
		ans[i] = int64(sum[r+1] - sum[j] + (j-l+1)*(j-l)/2)
	}

	return ans
}
