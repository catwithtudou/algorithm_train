package leetcode

import "strings"

func countPalindromicSubsequence(s string) (ans int) {

	for ch := byte('a'); ch <= 'z'; ch++ {
		i := strings.IndexByte(s, ch)
		if i < 0 {
			continue
		}
		j := strings.LastIndexByte(s, ch)
		if i+1 >= j {
			continue
		}

		has := [26]bool{}

		for _, mid := range s[i+1 : j] {
			if !has[mid-'a'] {
				has[mid-'a'] = true
				ans++
			}
		}
	}

	return
}
