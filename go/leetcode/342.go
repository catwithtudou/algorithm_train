package leetcode

func isPowerOfFour(n int) bool {
	if n <= 0 {
		return false
	}
	for ; n > 1; n /= 4 {
		if n%4 != 0 {
			return false
		}
	}

	return n == 1
}
