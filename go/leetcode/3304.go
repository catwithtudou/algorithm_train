package leetcode

import "math/bits"

func kthCharacter(k int) byte {
	return 'a' + byte(bits.OnesCount(uint(k-1)))
}
