package leetcode

import "math/bits"

func bitwiseComplement(n int) int {
	if n == 0 {
		return 1
	}
	w := bits.Len(uint(n))
	return (1<<w - 1) ^ n
}
