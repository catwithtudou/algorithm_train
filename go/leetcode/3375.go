package leetcode

import "sort"

func minOperations3375(nums []int, k int) (ans int) {
	n := len(nums)
	sort.Ints(nums)
	if nums[0] < k || nums[n-1] < k {
		return -1
	}

	for i := n - 1; i > 0; i-- {
		if nums[i] == k {
			continue
		}
		if nums[i] != nums[i-1] {
			ans++
		}
	}
	if nums[0] == k {
		return ans
	}
	return ans + 1
}
