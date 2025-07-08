package leetcode

import (
	"sort"

	"golang.org/x/exp/slices"
)

func maxValueII(events [][]int, k int) int {
	slices.SortFunc(events, func(a, b []int) int {
		return a[1] - b[1]
	})
	n := len(events)
	f := make([][]int, n+1)
	for i := range f {
		f[i] = make([]int, k+1)
	}

	for i, event := range events {
		start, value := event[0], event[2]
		p := sort.Search(i, func(j int) bool { return events[j][1] >= start })
		for j := 1; j <= k; j++ {
			f[i+1][j] = max(f[i][j], f[p][j-1]+value)
		}
	}

	return f[n][k]
}
