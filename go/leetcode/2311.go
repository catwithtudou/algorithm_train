package leetcode

import "math/bits"

func longestSubsequence(s string, k int) (ans int) {
	sm := 0
	n := len(s)
	bits := bits.Len(uint(k))
	for i := 0; i < len(s); i++ {
		if s[n-1-i] == '1' {
			if i < bits && sm+(1<<i) <= k {
				sm += 1 << i
				ans++
			}
		} else {
			ans++
		}
	}
	return
}
