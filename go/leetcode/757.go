package leetcode

import "sort"

func intersectionSizeTwo(intervals [][]int) (ans int) {
	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i][1] < intervals[j][1] || intervals[i][1] == intervals[j][1] && intervals[i][0] > intervals[j][0]
	})

	s, e := -1, -1

	for _, v := range intervals {
		a, b := v[0], v[1]
		if a <= s {
			continue
		}
		if a <= e {
			s, e = e, b
			ans += 1
		} else {
			s, e = b-1, b
			ans += 2
		}
	}

	return
}
