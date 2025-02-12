package leetcode

import (
	"slices"
	"sort"
)

func minimumSize(nums []int, maxOperations int) int {
	left, right := 1, slices.Max(nums)
	return left + sort.Search(right-left, func(m int) bool {
		m += left
		cnt := 0
		for _, x := range nums {
			cnt += (x - 1) / m
		}

		return cnt <= maxOperations
	})
}
