package leetcode

import "strings"

func checkOnesSegment(s string) bool {
	return !strings.Contains(s, "01")
}
