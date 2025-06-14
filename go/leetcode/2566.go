package leetcode

import (
	"strconv"
	"strings"
)

func minMaxDifference(num int) int {
	numStr := strconv.Itoa(num)
	mx := num
	for _, c := range numStr {
		if c != '9' {
			mx, _ = strconv.Atoi(strings.ReplaceAll(numStr, string(c), "9"))
			break
		}
	}
	mn, _ := strconv.Atoi(strings.ReplaceAll(numStr, numStr[:1], "0"))

	return mx - mn
}
