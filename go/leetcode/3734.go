package leetcode

import (
	"strings"

	"golang.org/x/exp/slices"
)

func lexPalindromicPermutation(s string, target string) string {
	left := make([]int, 26)
	for _, b := range s {
		left[b-'a']++
	}
	valid := func() bool {
		for _, c := range left {
			if c < 0 {
				return false
			}
		}
		return true
	}

	midCh := ""
	for i, c := range left {
		if c%2 == 0 {
			continue
		}
		if midCh != "" {
			return ""
		}
		midCh = string('a' + byte(i))
		left[i]--
	}

	n := len(s)
	for _, b := range target[:n/2] {
		left[b-'a'] -= 2
	}

	if valid() {
		leftS := target[:n/2]
		tmp := []byte(leftS)
		slices.Reverse(tmp)
		rightS := midCh + string(tmp)
		if rightS > target[n/2:] {
			return leftS + rightS
		}
	}

	for i := n/2 - 1; i >= 0; i-- {
		b := target[i] - 'a'
		left[b] += 2
		if !valid() {
			continue
		}

		for j := b + 1; j < 26; j++ {
			if left[j] == 0 {
				continue
			}

			left[j] -= 2
			ans := []byte(target[:i+1])
			ans[i] = 'a' + j

			for k, c := range left {
				ch := string('a' + byte(k))
				ans = append(ans, strings.Repeat(ch, c/2)...)
			}

			rightS := slices.Clone(ans)
			slices.Reverse(rightS)
			ans = append(ans, midCh...)
			ans = append(ans, rightS...)

			return string(ans)
		}
	}

	return ""
}
