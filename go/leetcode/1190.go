package leetcode

import "slices"

func reverseParentheses(s string) string {
	i := 0

	var dfs func() []byte

	dfs = func() (res []byte) {
		for i < len(s) {
			ch := s[i]
			i++
			if ch == ')' {
				slices.Reverse(res)
				return
			}
			if ch == '(' {
				res = append(res, dfs()...)
			} else {
				res = append(res, ch)
			}
		}
		return
	}

	return string(dfs())
}
