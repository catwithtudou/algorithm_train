package leetcode

func maxDepth(s string) (ans int) {

	depth := 0

	for _, ch := range s {
		if ch == '(' {
			depth++
			ans = max(ans, depth)
		} else if ch == ')' {
			depth--
		}

	}

	return
}
