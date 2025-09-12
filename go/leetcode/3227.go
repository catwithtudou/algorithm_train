package leetcode

import "strings"

func doesAliceWin(s string) bool {
	return strings.ContainsAny(s, "aeiou")
}
