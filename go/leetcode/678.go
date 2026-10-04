package leetcode

func checkValidString(s string) bool {
	// 未匹配左括号数量的最小值和最大值。
	low, high := 0, 0
	for i := range s {
		switch s[i] {
		case '(':
			low++
			high++
		case ')':
			low--
			high--
		case '*':
			low--
			high++
		}
		if high < 0 {
			return false
		}
		low = max(low, 0)
	}
	return low == 0
}
