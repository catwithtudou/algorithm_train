package leetcode

func hasAlternatingBits(n int) bool {
	x := (n >> 1) ^ n
	return (x+1)&x == 0
}
