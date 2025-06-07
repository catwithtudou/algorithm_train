package leetcode

import (
	"golang.org/x/exp/slices"
)

func clearStars(s string) string {
	stack := make([][]int, 26)
	for i, c := range s {
		if c != '*' {
			stack[c-'a'] = append(stack[c-'a'], i)
			continue
		}
		for j, st := range stack {
			if len(st) > 0 {
				stack[j] = st[:len(st)-1]
				break
			}
		}
	}

	idx := []int{}
	for _, p := range stack {
		idx = append(idx, p...)
	}
	slices.Sort(idx)

	ans := make([]byte, len(idx))
	for i, p := range idx {
		ans[i] = s[p]
	}
	return string(ans)
}
