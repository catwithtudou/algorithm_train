package leetcode

func countGoodNumbers(n int64) int {
	return newQuickPow(5, (int(n)+1)/2) * newQuickPow(4, int(n)/2) % mod
}

func newQuickPow(x, n int) int {
	res := 1
	for ; n > 0; n >>= 1 {
		if n&1 > 0 {
			res = res * x % mod
		}
		x = x * x % mod
	}
	return res
}
