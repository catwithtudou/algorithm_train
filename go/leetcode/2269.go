package leetcode

import "math"

func divisorSubstrings(num int, k int) (ans int) {
	mod := int(math.Pow10(k))
	for n := num; n >= mod/10; n /= 10 {
		x := n % mod
		if x > 0 && num%x == 0 {
			ans++
		}

	}
	return ans
}
