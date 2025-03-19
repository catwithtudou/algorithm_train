package leetcode

import "slices"

func findMatrix(nums []int) (ans [][]int) {
	slices.Sort(nums)
	repeat := 0
	ans = make([][]int, 0)
	for i := 0; i < len(nums); i++ {
		if len(ans) <= repeat {
			ans = append(ans, []int{})
		}
		ans[repeat] = append(ans[repeat], nums[i])
		if (i+1) < len(nums) && nums[i] == nums[i+1] {
			repeat++
		} else {
			repeat = 0
		}
	}

	return
}
