package leetcode

import "slices"

func reverseStr(s string, k int) string {
	S := []byte(s)
	n := len(S)
	for i := 0; i < n; i += 2 * k {
		slices.Reverse(S[i:min(i+k, n)])
	}
	return string(S)
}
