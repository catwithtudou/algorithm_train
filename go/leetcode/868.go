package leetcode

import "math/bits"

func binaryGap(n int) (ans int) {
	n /= n & -n * 2
	for n > 0 {
		gap := bits.TrailingZeros(uint(n)) + 1
		ans = max(ans, gap)
		n >>= gap
	}
	return ans
}
