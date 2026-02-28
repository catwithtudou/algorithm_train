package leetcode

import "math/bits"

func concatenatedBinary(n int) (ans int) {
	const mod int = 1e9 + 7
	for i := 1; i <= n; i++ {
		w := bits.Len(uint(i))
		ans = (ans<<w | i) % mod
	}
	return ans
}
