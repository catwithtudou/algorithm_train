package leetcode

import "math"

func maxDistance624(arrays [][]int) (ans int) {
	minNum, maxNum := math.MaxInt/2, math.MinInt/2
	for _, array := range arrays {
		x, y := array[0], array[len(array)-1]
		ans = max(ans, max(y-minNum, maxNum-x))
		minNum = min(minNum, x)
		maxNum = max(maxNum, y)
	}
	return
}
