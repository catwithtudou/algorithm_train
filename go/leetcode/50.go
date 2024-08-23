package leetcode

func myPow(x float64, n int) float64 {
	ans := 1.0
	if n < 0 {
		n = -n
		x = 1 / x
	}
	for ; n > 0; n /= 2 {
		if n%2 > 0 {
			ans = x * ans
		}
		x *= x
	}

	return ans
}
