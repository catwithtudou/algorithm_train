package leetcode

import (
	"slices"
	"sort"
)

func maxTwoEvents(events [][]int) (ans int) {
	slices.SortFunc(events, func(a, b []int) int { return a[1] - b[1] })

	type pair struct{ end, val int }
	st := []pair{{}}

	for _, e := range events {
		start, val := e[0], e[2]
		i := sort.Search(len(st), func(j int) bool { return st[j].end >= start }) - 1
		ans = max(ans, st[i].val+val)
		if val > st[len(st)-1].val {
			st = append(st, pair{end: e[1], val: val})
		}
	}

	return
}
