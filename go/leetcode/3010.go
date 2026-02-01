package leetcode

import "slices"

func minimumCost3010(nums []int) int {
	slices.Sort(nums[1:])
	return nums[0] + nums[1] + nums[2]
}
