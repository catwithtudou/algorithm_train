package leetcode

import (
	"cmp"
	"slices"
	"strings"
)

func makeLargestSpecial(s string) string {
	if len(s) <= 2 {
		return s
	}

	subStrings := []string{}
	diff, start := 0, 0
	for i, ch := range s {
		if ch == '1' {
			diff++
		} else if diff--; diff == 0 {
			subStrings = append(subStrings, "1"+makeLargestSpecial(s[start+1:i])+"0")
			start = i + 1
		}
	}

	slices.SortFunc(subStrings, func(a, b string) int { return cmp.Compare(b, a) })
	return strings.Join(subStrings, "")
}
