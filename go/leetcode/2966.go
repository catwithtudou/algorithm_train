package leetcode

import "slices"

func divideArray(nums []int, k int) [][]int {
	slices.Sort(nums)
	n := len(nums)
	ans := make([][]int, 0, n/3)
	for i := 0; i < n-2; i += 3 {
		if nums[i+1]-nums[i] > k || nums[i+2]-nums[i] > k {
			return [][]int{}
		}
		ans = append(ans, []int{nums[i], nums[i+1], nums[i+2]})
	}
	return ans
}
