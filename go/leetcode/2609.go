package leetcode

func findTheLongestBalancedSubstring(s string) int {
	ans, left, right := 0, 0, 0
	for i := 0; i < len(s); i++ {
		if s[i] == '0' {
			left++
		} else {
			right++
		}
		if left == right {
			ans = max(ans, left*2)
			left, right = 0, 0
		} else if left < right {
			left, right = 0, 0
		} else if left > right && right > 0 {
			ans = max(ans, right*2)
			if i+1 < len(s) && s[i+1] == '0' {
				left, right = 0, 0
			}
		}
	}

	return ans
}
