package leetcode

import "sort"

func countPairs(nums []int, target int) int {
	sort.Ints(nums)
	ans := 0
	for i := 1; i < len(nums); i++ {
		ans += sort.SearchInts(nums[0:i], target-nums[i])
	}
	return ans
}

func countPairsOthers(nums []int, target int) int {
	sort.Ints(nums)
	ans := 0
	for i, j := 0, len(nums)-1; i < j; i++ {
		for i < j && nums[i]+nums[j] >= target {
			j--
		}
		ans += j - i
	}
	return ans
}
