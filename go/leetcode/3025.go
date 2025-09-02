package leetcode

import (
	"math"
	"slices"
)

func numberOfPairs3025(points [][]int) (ans int) {
	slices.SortFunc(points, func(a, b []int) int {
		if a[0] != b[0] {
			return a[0] - b[0]
		}
		return b[1] - a[1]
	})
	for i, p := range points {
		y1 := p[1]
		maxY := math.MinInt
		for _, q := range points[i+1:] {
			y2 := q[1]
			if y2 <= y1 && y2 > maxY {
				maxY = y2
				ans++
			}
		}
	}

	return
}
