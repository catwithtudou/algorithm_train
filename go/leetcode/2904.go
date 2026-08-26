package leetcode

import "strings"

func shortestBeautifulSubstring(s string, k int) string {
	if strings.Count(s, "1") < k {
		return ""
	}
	for size := k; ; size++ {
		ans := ""
		for i := size; i < len(s); i++ {
			t := s[i-size : i]
			if (ans == "" || t < ans) && strings.Count(t, "1") == k {
				ans = t
			}
		}
		if ans != "" {
			return ans
		}
	}
}
