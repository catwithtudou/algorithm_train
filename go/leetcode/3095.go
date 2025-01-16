package leetcode

import "math"

func minimumSubarrayLength(nums []int, k int) int {
	ans := math.MaxInt
	for i, x := range nums {
		if x >= k {
			return 1
		}
		for j := i - 1; j >= 0 && nums[j]|x != nums[j]; j-- {
			nums[j] |= x
			if nums[j] >= k {
				ans = min(ans, i-j+1)
			}
		}

	}

	if ans == math.MaxInt {
		return -1
	}

	return ans
}
