package leetcode

import "math"

func minMoves(nums []int, limit int) int {
	n := len(nums)
	diff := make([]int, limit*2+2)
	for i, x := range nums[:n/2] {
		y := nums[n-1-i]
		l := min(x, y) + 1
		r := max(x, y) + limit

		diff[2] += 2
		diff[l] -= 2

		diff[l]++
		diff[r+1]--

		diff[x+y]--
		diff[x+y+1]++

		diff[r+1] += 2
	}

	ans := math.MaxInt
	sumD := 0
	for _, v := range diff[2 : limit*2+1] {
		sumD += v
		ans = min(ans, sumD)
	}

	return ans
}
