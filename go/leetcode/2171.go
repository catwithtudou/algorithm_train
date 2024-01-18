package leetcode

import "sort"

func minimumRemoval(beans []int) int64 {
	sum := 0
	mx := 0
	sort.Ints(beans)
	for i, v := range beans {
		sum += v
		mx = max(mx, (len(beans)-i)*v)
	}
	return int64(sum - mx)
}
