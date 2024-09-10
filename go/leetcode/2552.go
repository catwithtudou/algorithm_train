package leetcode

import "golang.org/x/exp/slices"

func countQuadruplets(nums []int) (ans int64) {
	n := len(nums)
	great := make([][]int, n)
	great[n-1] = make([]int, n+1)
	for i := n - 2; i >= 2; i-- {
		great[i] = slices.Clone(great[i+1])
		for x := 1; x < nums[i+1]; x++ {
			great[i][x]++
		}
	}

	less := make([]int, n+1)
	for i := 1; i < n-2; i++ {
		for x := nums[i-1] + 1; x <= n; x++ {
			less[x]++
		}
		for k := i + 1; k < n-1; k++ {
			if nums[i] > nums[k] {
				ans += int64(great[k][nums[i]] * less[nums[k]])
			}
		}
	}

	return
}
