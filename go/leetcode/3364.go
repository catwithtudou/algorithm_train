package leetcode

import "math"

func minimumSumSubarray(nums []int, l int, r int) int {
	ans := math.MaxInt
	n := len(nums)
	for i := range n - l + 1 {
		s := 0
		for j := i; j < n && j-i+1 <= r; j++ {
			s += nums[j]
			if s > 0 && j-i+1 >= l {
				ans = min(ans, s)
			}
		}
	}
	if ans == math.MaxInt {
		return -1
	}
	return ans
}
