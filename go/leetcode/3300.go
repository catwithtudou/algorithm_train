package leetcode

import "math"

func minElement(nums []int) int {
	ans := math.MaxInt

	for _, num := range nums {
		x := 0
		for num > 0 {
			x += num % 10
			num /= 10
		}
		ans = min(ans, x)
	}

	return ans
}
