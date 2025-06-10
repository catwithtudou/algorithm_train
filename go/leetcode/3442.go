package leetcode

import "math"

func maxDifference(s string) int {
	cnt := make([]int, 26)
	for _, c := range s {
		cnt[c-'a']++
	}
	maxJ, minO := 0, math.MaxInt
	for _, c := range cnt {
		if c%2 > 0 {
			maxJ = max(maxJ, c)
		} else if c > 0 {
			minO = min(minO, c)
		}
	}
	return maxJ - minO
}
