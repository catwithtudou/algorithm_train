package leetcode

func minLength(s string) int {
	stack := make([]byte, 0, len(s))
	for i := range s {
		stack = append(stack, s[i])
		m := len(stack)
		if m < 2 {
			continue
		}
		if (stack[m-1] == 'B' && stack[m-2] == 'A') || (stack[m-1] == 'D' && stack[m-2] == 'C') {
			stack = stack[:m-2]
		}
	}

	return len(stack)
}
