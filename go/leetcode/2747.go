package leetcode

import "slices"

func countServers(n int, logs [][]int, x int, queries []int) []int {
	slices.SortFunc(logs, func(a, b []int) int { return a[1] - b[1] })
	idx := make([]int, len(queries))
	for i := range idx {
		idx[i] = i
	}
	slices.SortFunc(idx, func(i, j int) int { return queries[i] - queries[j] })
	cnt := make([]int, n+1)
	notR := n
	l, r := 0, 0

	for _, qid := range idx {
		q := queries[qid]

		for r < len(logs) && logs[r][1] <= q {
			i := logs[r][0]
			if cnt[i] == 0 {
				notR--
			}
			cnt[i]++
			r++
		}

		for l < len(logs) && logs[l][1] < q-x {
			i := logs[l][0]
			cnt[i]--
			if cnt[i] == 0 {
				notR++
			}
			l++
		}

		queries[qid] = notR
	}

	return queries
}
