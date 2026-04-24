package leetcode

import "strings"

func furthestDistanceFromOrigin(moves string) (ans int) {
	lCnt := strings.Count(moves, "L")
	rCnt := strings.Count(moves, "R")
	return abs(lCnt-rCnt) + len(moves) - lCnt - rCnt
}
