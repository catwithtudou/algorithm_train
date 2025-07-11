﻿package leetcode

import (
	"slices"
	"sort"
)

func minimizeMax(nums []int, p int) int {
	slices.Sort(nums)
	n := len(nums)
	return sort.Search(nums[n-1]-nums[0], func(mx int) bool {
		cnt := 0
		for i := 0; i < n-1; i++ {
			if nums[i+1]-nums[i] <= mx {
				cnt++
				i++
			}
		}
		return cnt >= p
	})
}
