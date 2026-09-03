package leetcode

import "math"

func uniformArray3876(nums1 []int) bool {
	mn := [2]int{math.MaxInt, math.MaxInt}

	for _, x := range nums1 {
		mn[x&1] = min(mn[x&1], x)
	}

	return mn[1] == math.MaxInt || mn[0] > mn[1]
}
