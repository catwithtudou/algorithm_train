package leetcode

func sumIndicesWithKSetBits(nums []int, k int) int {
	ans := 0
	for i, v := range nums {
		if oneBitCount(i) == k {
			ans += v
		}
	}
	return ans
}

func oneBitCount(a int) int {
	c := 0
	for a > 0 {
		a &= a - 1
		c++
	}

	return c
}
