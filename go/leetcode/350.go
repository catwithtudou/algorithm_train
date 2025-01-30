package leetcode

import "golang.org/x/exp/slices"

func intersect(nums1 []int, nums2 []int) (ans []int) {
	slices.Sort(nums1)
	slices.Sort(nums2)

	i, j := 0, 0

	for i < len(nums1) && j < len(nums2) {
		x, y := nums1[i], nums2[j]
		if x == y {
			ans = append(ans, x)
			i++
			j++
		} else if x < y {
			i++
		} else {
			j++
		}
	}

	return
}
