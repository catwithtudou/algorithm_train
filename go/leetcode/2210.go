package leetcode

import "slices"

func countHillValley(nums []int) (ans int) {
	nums = slices.Compact(nums)
	for i := 1; i < len(nums)-1; i++ {
		if (nums[i-1] < nums[i]) == (nums[i] > nums[i+1]) {
			ans++
		}
	}
	return
}
