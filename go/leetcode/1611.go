package leetcode

import "math/bits"

func minimumOneBitOperations(n int) int {
	if n == 0 {
		return 0
	}
	k := bits.Len(uint(n))
	return (1 << k) - 1 - minimumOneBitOperations(n-(1<<(k-1)))
}
