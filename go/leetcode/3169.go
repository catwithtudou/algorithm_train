package leetcode

import "slices"

func countDays(days int, meetings [][]int) int {
	slices.SortFunc(meetings, func(a, b []int) int {
		return a[0] - b[0]
	})
	start, end := 1, 0
	for _, p := range meetings {
		if p[0] > end {
			days -= end - start + 1
			start = p[0]
		}
		end = max(end, p[1])
	}
	return days - (end - start + 1)
}
