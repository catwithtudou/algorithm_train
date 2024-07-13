package leetcode

import (
	"math/bits"

	"golang.org/x/exp/slices"
)

func canSortArray(nums []int) bool {
	n := len(nums)
	for i := 0; i < n; {
		start := i
		ones := bits.OnesCount(uint(nums[start]))
		i++
		for i < n && ones == bits.OnesCount(uint(nums[i])) {
			i++
		}
		slices.Sort(nums[start:i])
	}
	return slices.IsSorted(nums)
}

func canSortArray3011(nums []int) bool {
	n := len(nums)
	preMax := 0
	for i := 0; i < n; {
		mx := 0
		ones := bits.OnesCount(uint(nums[i]))
		for ; i < n && ones == bits.OnesCount(uint(nums[i])); i++ {
			if nums[i] < preMax {
				return false
			}
			mx = max(mx, nums[i])
		}
		preMax = mx
	}
	return true
}
