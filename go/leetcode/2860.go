package leetcode

import "golang.org/x/exp/slices"

func countWays2860(nums []int) int {
	slices.Sort(nums)
	ans := 0
	if nums[0] > 0 {
		ans++
	}
	for i := 1; i < len(nums); i++ {
		if nums[i-1] < i && nums[i] > i {
			ans++
		}
	}

	return ans + 1
}
