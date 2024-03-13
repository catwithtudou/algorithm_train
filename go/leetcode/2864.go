package leetcode

import "strings"

func maximumOddBinaryNumber(s string) string {
	oneCnt := strings.Count(s, "1")
	return strings.Repeat("1", oneCnt-1) + strings.Repeat("0", len(s)-oneCnt) + "1"
}
