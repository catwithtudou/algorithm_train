package leetcode

func removeOuterParentheses(s string) string {
	depth := 0
	ans := []byte{}
	for _, ch := range s {
		if ch == '(' {
			if depth > 0 {
				ans = append(ans, byte(ch))
			}
			depth++
		} else {
			depth--
			if depth > 0 {
				ans = append(ans, byte(ch))
			}
		}
	}
	return string(ans)
}
