package leetcode

import "sort"

func canPartitionKSubsets(nums []int, k int) bool {
	target := 0
	for _, v := range nums {
		target += v
	}
	if target%k != 0 {
		return false
	}
	target = target / k

	bucket := make([]int, k)
	var dfs func(int) bool
	dfs = func(i int) bool {
		if i == len(nums) {
			return true
		}
		for j := 0; j < k; j++ {
			if j > 0 && bucket[j] == bucket[j-1] {
				continue
			}
			if bucket[j]+nums[i] > target {
				continue
			}
			bucket[j] += nums[i]
			if dfs(i + 1) {
				return true
			}
			bucket[j] -= nums[i]

		}
		return false
	}

	sort.Sort(sort.Reverse(sort.IntSlice(nums)))

	return dfs(0)
}
