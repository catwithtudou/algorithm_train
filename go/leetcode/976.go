package leetcode

import "slices"

func largestPerimeter(nums []int) (ans int) {

	slices.Sort(nums)

	for i := len(nums) - 1; i >= 2; i-- {
		if nums[i-2]+nums[i-1] > nums[i] {
			return nums[i-2] + nums[i-1] + nums[i]
		}
	}

	return
}
