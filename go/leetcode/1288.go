package leetcode

import (
	"cmp"

	"golang.org/x/exp/slices"
)

func removeCoveredIntervals(intervals [][]int) (ans int) {

	slices.SortFunc(intervals, func(a, b []int) int {
		return cmp.Or(a[0]-b[0], b[1]-a[1])
	})

	maxRight := 0

	for _, i := range intervals {
		if i[1] > maxRight {
			maxRight = i[1]
			ans++
		}
	}

	return
}
