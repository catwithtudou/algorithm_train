package leetcode

// 提交到 LeetCode 时将函数名改为 isValid，避免与 2056 题的辅助函数重名。
func isValid20(s string) bool {
	stack := make([]byte, 0, len(s))
	for i := range s {
		switch s[i] {
		case '(':
			stack = append(stack, ')')
		case '[':
			stack = append(stack, ']')
		case '{':
			stack = append(stack, '}')
		default:
			if len(stack) == 0 || stack[len(stack)-1] != s[i] {
				return false
			}
			stack = stack[:len(stack)-1]
		}
	}
	return len(stack) == 0
}
