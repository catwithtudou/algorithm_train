package leetcode

import "strings"

func findWordsContaining(words []string, x byte) (ans []int) {
	for i, s := range words {
		if strings.IndexByte(s, x) >= 0 {
			ans = append(ans, i)
		}
	}
	return
}
