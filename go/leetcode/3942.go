package leetcode

import "math"

func minOperations3942(nums []int) int {
	n := len(nums)

	cnt, p := 0, 0

	for i := 1; i < n && cnt < 2; i++ {
		if nums[i-1] > nums[i] {
			cnt++
			p = i
		}
	}

	if cnt == 0 {
		return 0
	}

	ans := math.MaxInt

	if cnt == 1 && nums[0] > nums[n-1] {
		ans = min(p, n-p+2)
	}

	cnt, p = 0, 0

	for i := 1; i < n && cnt < 2; i++ {
		if nums[i-1] < nums[i] {
			cnt++
			p = i
		}
	}

	if cnt == 0 {
		return 1
	}

	if cnt == 1 && nums[0] < nums[n-1] {
		ans = min(ans, min(p+1, n-p+1))
	}

	if ans == math.MaxInt {
		return -1
	}

	return ans
}
