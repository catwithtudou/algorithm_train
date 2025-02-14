package leetcode

import (
	"slices"
	"sort"
)

func maxDistance(position []int, m int) int {
	slices.Sort(position)
	return sort.Search((position[len(position)-1]-position[0])/(m-1), func(d int) bool {
		d++
		cnt, pre := 1, position[0]
		for _, x := range position[1:] {
			if x >= d+pre {
				cnt++
				pre = x
			}
		}

		return cnt < m
	})

}
