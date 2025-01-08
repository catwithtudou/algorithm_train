package leetcode

import "strings"

func largestGoodInteger(num string) string {
	mx := byte(0)
	for i := range len(num) - 2 {
		d := num[i]
		if d > mx && d == num[i+1] && d == num[i+2] {
			mx = d
		}
	}

	if mx == 0 {
		return ""
	}

	return strings.Repeat(string(mx), 3)
}
