package leetcode

import "slices"

func minimumCostII(m int, n int, horizontalCut []int, verticalCut []int) (ans int64) {
	slices.SortFunc(horizontalCut, func(a, b int) int {
		return b - a
	})
	slices.SortFunc(verticalCut, func(a, b int) int {
		return b - a
	})
	i, j := 0, 0
	h, v := 1, 1
	for i < m-1 || j < n-1 {
		if j == n-1 || (i < m-1 && horizontalCut[i] > verticalCut[j]) {
			ans += int64(horizontalCut[i] * v)
			i++
			h++
		} else {
			ans += int64(verticalCut[j] * h)
			j++
			v++
		}
	}
	return
}
