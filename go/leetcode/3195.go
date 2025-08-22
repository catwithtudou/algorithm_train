package leetcode

import "math"

func minimumArea(grid [][]int) (ans int) {
	left, right := math.MaxInt, 0
	top, bottom := math.MaxInt, 0
	for i, row := range grid {
		for j, v := range row {
			if v == 1 {
				left = min(left, j)
				right = max(right, j)
				top = min(top, i)
				bottom = max(bottom, i)
			}
		}
	}
	return (right - left + 1) * (bottom - top + 1)
}
