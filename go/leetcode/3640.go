package leetcode

import "math"

func maxSumTrionic(nums []int) int64 {
	n := len(nums)
	ans := math.MinInt

	for i := 0; i < n; {
		start := i
		for i++; i < n && nums[i-1] < nums[i]; i++ {
		}
		if i == start+1 {
			continue
		}

		peak := i - 1
		res := nums[peak] + nums[peak-1]
		for ; i < n && nums[i-1] > nums[i]; i++ {
			res += nums[i]
		}

		if i == peak+1 || i == n || nums[i-1] == nums[i] {
			continue
		}

		bottom := i - 1
		res += nums[i]
		maxS, s := 0, 0
		for i++; i < n && nums[i-1] < nums[i]; i++ {
			s += nums[i]
			maxS = max(maxS, s)
		}
		res += maxS

		maxS, s = 0, 0
		for j := peak - 2; j >= start; j-- {
			s += nums[j]
			maxS = max(maxS, s)
		}
		res += maxS

		ans = max(ans, res)

		i = bottom
	}

	return int64(ans)
}
