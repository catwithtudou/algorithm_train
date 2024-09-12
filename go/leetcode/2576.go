package leetcode

import (
	"sort"

	"golang.org/x/exp/slices"
)

func maxNumOfMarkedIndices(nums []int) int {
	slices.Sort(nums)
	n := len(nums)
	pairs := sort.Search(n/2, func(k int) bool {
		k++
		for i, x := range nums[:k] {
			if 2*x > nums[n-k+i] {
				return true
			}
		}
		return false
	})
	return pairs * 2
}
