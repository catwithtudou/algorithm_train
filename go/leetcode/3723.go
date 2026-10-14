package leetcode

import "strings"

func maxSumOfSquares(num int, sum int) string {
	if num*9 < sum {
		return ""
	}

	ans := strings.Repeat("9", sum/9)

	if sum%9 > 0 {
		ans += string('0' + byte(sum%9))
	}

	return ans + strings.Repeat("0", num-len(ans))
}
