package leetcode

import "slices"

func maximumBeautyIII(items [][]int, queries []int) []int {
	slices.SortFunc(items, func(a, b []int) int { return a[0] - b[0] })
	idx := make([]int, len(queries))
	for i := range queries {
		idx[i] = i
	}

	slices.SortFunc(idx, func(i, j int) int { return queries[i] - queries[j] })

	ans := make([]int, len(queries))
	j, maxBea := 0, 0
	for _, i := range idx {
		q := queries[i]
		for j < len(items) && items[j][0] <= q {
			maxBea = max(maxBea, items[j][1])
			j++
		}
		ans[i] = maxBea
	}
	return ans
}
