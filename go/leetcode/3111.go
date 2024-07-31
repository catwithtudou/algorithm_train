package leetcode

import "golang.org/x/exp/slices"

func minRectanglesToCoverPoints(points [][]int, w int) int {
	ans := 0
	slices.SortFunc(points, func(a, b []int) int { return a[0] - b[0] })
	x2 := -1
	for _, p := range points {
		if p[0] > x2 {
			ans++
			x2 = p[0] + w
		}
	}
	return ans
}
