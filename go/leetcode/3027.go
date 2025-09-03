package leetcode

import (
	"cmp"
	"math"
	"slices"
)

func numberOfPairsII(points [][]int) (ans int) {
	slices.SortFunc(points, func(a, b []int) int {
		return cmp.Or(a[0]-b[0], b[1]-a[1])
	})
	for i, point := range points {
		y1 := point[1]
		maxY := math.MinInt
		for _, p := range points[i+1:] {
			y2 := p[1]
			if y2 <= y1 && y2 > maxY {
				maxY = y2
				ans++
			}
			if maxY == y1 {
				break
			}
		}
	}
	return
}
