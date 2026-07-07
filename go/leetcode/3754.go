package leetcode

func sumAndMultiply(n int) int64 {
	sum, pow10, num := 0, 1, 0
	for ; n > 0; n /= 10 {
		d := n % 10
		if d > 0 {
			num += d * pow10
			sum += d
			pow10 *= 10
		}
	}
	return int64(sum) * int64(num)
}
