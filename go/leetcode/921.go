package leetcode

func minAddToMakeValid(s string) int {
	left, ans := 0, 0
	for i := range s {
		if s[i] == '(' {
			left++
		} else if left > 0 {
			left--
		} else {
			// 当前右括号没有匹配的左括号，需要补一个。
			ans++
		}
	}
	return ans + left
}
