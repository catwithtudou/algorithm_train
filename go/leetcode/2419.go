package leetcode

import "slices"

func longestSubarray(nums []int) (ans int) {

	maxCnt := slices.Max(nums)
	cnt := 0

	for _, x := range nums {
		if x == maxCnt {
			cnt++
		} else {
			cnt = 0
		}
		ans = max(ans, cnt)
	}

	return
}
