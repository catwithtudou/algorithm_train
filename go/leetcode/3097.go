package leetcode

import "math"

func minimumSubarrayLengthII(nums []int, k int) int {
	ans := math.MaxInt
	var bottom, left, right_or int
	for right, x := range nums {
		right_or |= x
		for left <= right && nums[left]|right_or >= k {
			ans = min(ans, right-left+1)
			left++
			if bottom < left {
				for j := right - 1; j >= left; j-- {
					nums[j] |= nums[j+1]
				}
				bottom = right
				right_or = 0
			}
		}
	}

	if ans == math.MaxInt {
		return -1
	}
	return ans
}
