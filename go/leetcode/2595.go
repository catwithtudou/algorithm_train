package leetcode

func evenOddBit(n int) []int {
	ans := make([]int, 2)
	for i := 0; n > 0; n >>= 1 {
		ans[i] += n & 1
		i ^= 1
	}
	return ans
}
