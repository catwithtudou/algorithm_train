package leetcode

import "strings"

func lexGreaterPermutation(s string, target string) string {
	left := make([]int, 26)
	for i, b := range s {
		left[b-'a']++
		left[target[i]-'a']--
	}

next:
	for i := len(s) - 1; i >= 0; i-- {
		b := target[i] - 'a'
		left[b]++
		for _, c := range left {
			if c < 0 {
				continue next
			}
		}

		for j := b + 1; j < 26; j++ {
			if left[j] == 0 {
				continue
			}

			left[j]--
			ans := []byte(target[:i+1])
			ans[i] = 'a' + j

			for k, c := range left {
				ch := string('a' + byte(k))
				ans = append(ans, strings.Repeat(ch, c)...)
			}

			return string(ans)
		}
	}
	return ""
}
