package leetcode

import (
	"golang.org/x/exp/slices"
)

func minimumAddedInteger(nums1 []int, nums2 []int) int {
	slices.Sort(nums1)
	slices.Sort(nums2)

	for i := 2; i > 0; i-- {
		x := nums2[0] - nums1[i]
		j := 0
		for _, v := range nums1[i:] {
			if nums2[j] == v+x {
				j++
				if j == len(nums2) {
					return x
				}
			}
		}
	}

	return nums2[0] - nums1[0]
}
