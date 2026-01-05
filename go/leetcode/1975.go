package leetcode

import "math"

func maxMatrixSum(matrix [][]int) (ans int64) {
	total, negCnt, mn := 0, 0, math.MaxInt

	for _, row := range matrix {
		for _, x := range row {
			if x < 0 {
				negCnt++
				x = -x
			}
			mn = min(mn, x)
			total += x
		}
	}

	if negCnt%2 > 0 {
		total -= 2 * mn
	}

	return int64(total)
}
