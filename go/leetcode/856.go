package leetcode

func scoreOfParentheses(s string) (ans int) {
	depth := 0
	for i := range s {
		if s[i] == '(' {
			depth++
		} else {
			depth--
			if s[i-1] == '(' {
				// 每个最内层的 () 被外层括号翻倍 depth 次。
				ans += 1 << depth
			}
		}
	}
	return
}
