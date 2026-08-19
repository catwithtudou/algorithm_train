package leetcode

import "slices"

func largestInteger(nums []int, k int) int {
	n := len(nums)
	if k == n {
		return slices.Max(nums)
	}

	if k == 1 {
		numShow := make(map[int]int)
		for _, x := range nums {
			numShow[x]++
		}
		ans := -1
		for i, x := range numShow {
			if x == 1 {
				ans = max(ans, i)
			}
		}
		return ans
	}

	f := func(a []int, x int) int {
		if slices.Contains(a, x) {
			return -1
		}
		return x
	}

	return max(f(nums[1:], nums[0]), f(nums[:n-1], nums[n-1]))
}
