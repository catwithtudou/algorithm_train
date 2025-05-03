package leetcode

import "math"

func minDominoRotations(tops []int, bottoms []int) int {
	minRot := func(target int) int {
		toTop, toBottom := 0, 0
		for i, x := range tops {
			y := bottoms[i]
			if x != target && y != target {
				return math.MaxInt
			}
			if x != target {
				toTop++
			} else if y != target {
				toBottom++
			}
		}
		return min(toTop, toBottom)
	}

	ans := min(minRot(tops[0]), minRot(bottoms[0]))
	if ans == math.MaxInt {
		return -1
	}
	return ans
}
