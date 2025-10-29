package leetcode

import "math/bits"

func smallestNumber(n int) int {
	return 1<<bits.Len(uint(n)) - 1
}
