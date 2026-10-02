package leetcode

func longestValidParentheses(s string) (ans int) {
	stack := []int{-1}
	for i := range s {
		if s[i] == '(' {
			stack = append(stack, i)
		} else {
			stack = stack[:len(stack)-1]
			if len(stack) == 0 {
				// 无法匹配的右括号作为下一段的左边界。
				stack = append(stack, i)
			} else {
				ans = max(ans, i-stack[len(stack)-1])
			}
		}
	}
	return
}
