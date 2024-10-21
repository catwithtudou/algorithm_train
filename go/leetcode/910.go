package leetcode

import "golang.org/x/exp/slices"

func smallestRangeII(nums []int, k int) int {
	slices.Sort(nums)
	n := len(nums)
	ans := nums[n-1] - nums[0]
	for i := 1; i < n; i++ {
		mx := max(nums[i-1]+k, nums[n-1]-k)
		mn := min(nums[0]+k, nums[i]-k)
		ans = min(ans, mx-mn)
	}

	return ans
}
