package leetcode

import "golang.org/x/exp/slices"

func minimumTime(nums1 []int, nums2 []int, x int) int {
	n := len(nums1)
	sum1, sum2 := 0, 0
	id := make([]int, n)
	for i := range id {
		id[i] = i
		sum1 += nums1[i]
		sum2 += nums2[i]
	}

	slices.SortFunc(id, func(i, j int) int {
		return nums2[i] - nums2[j]
	})

	f := make([]int, n+1)
	for i, p := range id {
		a, b := nums1[p], nums2[p]
		for j := i + 1; j > 0; j-- {
			f[j] = max(f[j], f[j-1]+a+b*j)
		}
	}

	for t, v := range f {
		if sum1+sum2*t-v <= x {
			return t
		}
	}

	return -1
}
