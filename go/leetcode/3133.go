package leetcode

func minEnd(n int, x int) int64 {
	i, j := 0, 0
	n--
	for n>>j > 0 {
		if x>>i&1 == 0 {
			x |= (n >> j & 1) << i
			j++
		}
		i++
	}
	return int64(x)
}
