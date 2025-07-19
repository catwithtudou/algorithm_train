package leetcode

import (
	"slices"
	"strings"
)

func removeSubfolders(folder []string) []string {
	slices.Sort(folder)
	ans := folder[:1]
	for _, s := range folder[1:] {
		last := ans[len(ans)-1]
		if !strings.HasPrefix(s, last) || s[len(last)] != '/' {
			ans = append(ans, s)
		}
	}
	return ans
}
